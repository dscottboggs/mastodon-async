use log::{debug, error, trace};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Client;
use uuid::Uuid;

use crate::{
    entities::forms, entities::prelude::*, helpers::read_response::read_response, Data, Error,
    Mastodon, Result,
};

const DEFAULT_REDIRECT_URI: &str = "urn:ietf:wg:oauth:2.0:oob";

/// Handles registering your mastodon app to your instance. It is recommended
/// you cache your data struct to avoid registering on every run.
#[derive(Debug, Clone)]
pub struct Registration {
    base: String,
    client: Client,
    app_builder: forms::ApplicationBuilder,
    force_login: bool,
}

#[derive(Serialize, Deserialize)]
struct OAuth {
    client_id: String,
    client_secret: String,
    #[serde(default = "default_redirect_uri")]
    redirect_uri: String,
}

fn default_redirect_uri() -> String {
    DEFAULT_REDIRECT_URI.to_string()
}

#[derive(Serialize, Deserialize)]
struct AccessToken {
    access_token: String,
}

impl Registration {
    /// Construct a new registration process to the instance of the `base` url.
    /// ```
    /// use mastodon_async::prelude::*;
    ///
    /// let registration = Registration::new("https://botsin.space");
    /// ```
    pub fn new<I: Into<String>>(base: I) -> Self {
        Registration::new_with_client(base, Client::new())
    }

    /// Construct a new registration process to the instance of the `base` url,
    /// using the provided [Client].
    /// ```
    /// use mastodon_async::prelude::*;
    ///
    /// let client = reqwest::Client::builder().user_agent("my cool app").build().unwrap();
    /// let registration = Registration::new_with_client("https://botsin.space", client);
    /// ```
    pub fn new_with_client<I: Into<String>>(base: I, client: Client) -> Self {
        Registration {
            base: base.into(),
            client,
            app_builder: forms::ApplicationBuilder::default(),
            force_login: false,
        }
    }
}

impl Registration {
    #[allow(dead_code)]
    pub(crate) fn with_sender<I: Into<String>>(base: I) -> Self {
        Registration {
            base: base.into(),
            client: Client::new(),
            app_builder: forms::ApplicationBuilder::default(),
            force_login: false,
        }
    }

    /// Sets the name of this app
    ///
    /// This is required, and if this isn't set then the
    /// [`entities::forms::ApplicationBuilder::build`] method will fail
    pub fn client_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.app_builder.client_name(name);
        self
    }

    /// Sets the redirect uris that this app uses
    pub fn redirect_uris(&mut self, uris: impl Into<String>) -> &mut Self {
        self.app_builder.redirect_uris(uris);
        self
    }

    /// Sets the scopes that this app requires
    ///
    /// The default for an app is Scopes::Read
    pub fn scopes(&mut self, scopes: Scopes) -> &mut Self {
        self.app_builder.scopes(scopes);
        self
    }

    /// Sets the optional "website" to register the app with
    pub fn website(&mut self, website: impl Into<String>) -> &mut Self {
        self.app_builder.website(website);
        self
    }

    /// Forces the user to re-login (useful if you need to re-auth as a
    /// different user on the same instance
    pub fn force_login(&mut self, force_login: bool) -> &mut Self {
        self.force_login = force_login;
        self
    }

    /// Register the given application
    ///
    /// ```no_run
    /// use mastodon_async::prelude::*;
    ///
    /// tokio_test::block_on(async {
    ///     let mut app = forms::Application::builder();
    ///     app.client_name("mastodon-async_test");
    ///
    ///     let registration = Registration::new("https://botsin.space")
    ///         .register(app)
    ///         .await
    ///         .unwrap();
    ///     let url = registration.authorize_url().unwrap();
    ///     // Here you now need to open the url in the browser
    ///     // And handle a the redirect url coming back with the code.
    ///     let code = String::from("RETURNED_FROM_BROWSER");
    ///     let mastodon = registration.complete(&code).await.unwrap();
    ///
    ///     println!("{:?}", mastodon.get_home_timeline().await.unwrap().initial_items);
    /// });
    /// ```
    pub async fn register<I: TryInto<forms::Application>>(&mut self, app: I) -> Result<Registered>
    where
        Error: From<<I as TryInto<forms::Application>>::Error>,
    {
        let app = app.try_into()?;
        let oauth = self.send_app(&app).await?;

        Ok(Registered {
            base: self.base.clone(),
            client: self.client.clone(),
            client_id: oauth.client_id,
            client_secret: oauth.client_secret,
            redirect: oauth.redirect_uri,
            scopes: app.scopes().clone(),
            force_login: self.force_login,
        })
    }

    /// Register the application with the server from the `base` url.
    ///
    /// ```no_run
    /// use mastodon_async::prelude::*;
    ///
    /// tokio_test::block_on(async {
    ///     let registration = Registration::new("https://botsin.space")
    ///         .client_name("mastodon-async_test")
    ///         .build()
    ///         .await
    ///         .unwrap();
    ///     let url = registration.authorize_url().unwrap();
    ///     // Here you now need to open the url in the browser
    ///     // And handle a the redirect url coming back with the code.
    ///     let code = String::from("RETURNED_FROM_BROWSER");
    ///     let mastodon = registration.complete(&code).await.unwrap();
    ///
    ///     println!("{:?}", mastodon.get_home_timeline().await.unwrap().initial_items);
    /// });
    /// ```
    pub async fn build(&mut self) -> Result<Registered> {
        let app: forms::Application = self.app_builder.clone().build()?;
        let oauth = self.send_app(&app).await?;

        Ok(Registered {
            base: self.base.clone(),
            client: self.client.clone(),
            client_id: oauth.client_id,
            client_secret: oauth.client_secret,
            redirect: oauth.redirect_uri,
            scopes: app.scopes().clone(),
            force_login: self.force_login,
        })
    }

    async fn send_app(&self, app: &forms::Application) -> Result<OAuth> {
        let url = format!("{}/api/v1/apps", self.base);
        let call_id = Uuid::new_v4();
        debug!(url = url, app:serde = app, call_id:? = call_id; "registering app");
        let response = self.client.post(&url).json(&app).send().await?;

        match response.error_for_status() {
            Ok(response) => {
                let response = read_response(response).await?;
                debug!(
                    response:serde = response, app:serde = app,
                    url = url, method = stringify!($method),
                    call_id:? = call_id;
                    "received API response"
                );
                Ok(response)
            }
            Err(err) => {
                error!(
                    err:? = err, url = url, method = stringify!($method),
                    call_id:? = call_id;
                    "error making API request"
                );
                Err(err.into())
            }
        }
    }
}

impl Registered {
    /// Skip having to retrieve the client id and secret from the server by
    /// creating a `Registered` struct directly
    ///
    /// // Example
    ///
    /// ```no_run
    /// use mastodon_async::{prelude::*, registration::Registered};
    ///
    /// tokio_test::block_on(async {
    ///     let registration = Registered::from_parts(
    ///         "https://example.com",
    ///         "the-client-id",
    ///         "the-client-secret",
    ///         "https://example.com/redirect",
    ///         Scopes::read_all(),
    ///         false,
    ///     );
    ///     let url = registration.authorize_url().unwrap();
    ///     // Here you now need to open the url in the browser
    ///     // And handle a the redirect url coming back with the code.
    ///     let code = String::from("RETURNED_FROM_BROWSER");
    ///     let mastodon = registration.complete(&code).await.unwrap();
    ///
    ///     println!("{:?}", mastodon.get_home_timeline().await.unwrap().initial_items);
    /// });
    /// ```
    pub fn from_parts(
        base: &str,
        client_id: &str,
        client_secret: &str,
        redirect: &str,
        scopes: Scopes,
        force_login: bool,
    ) -> Registered {
        Registered {
            base: base.to_string(),
            client: Client::new(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect: redirect.to_string(),
            scopes,
            force_login,
        }
    }
}

impl Registered {
    /// Returns the parts of the `Registered` struct that can be used to
    /// recreate another `Registered` struct
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::{prelude::*, registration::Registered};
    ///
    /// let orig_base = "https://example.social";
    /// let orig_client_id = "some-client_id";
    /// let orig_client_secret = "some-client-secret";
    /// let orig_redirect = "https://example.social/redirect";
    /// let orig_scopes = Scopes::all();
    /// let orig_force_login = false;
    ///
    /// let registered = Registered::from_parts(
    ///     orig_base,
    ///     orig_client_id,
    ///     orig_client_secret,
    ///     orig_redirect,
    ///     orig_scopes.clone(),
    ///     orig_force_login,
    /// );
    ///
    /// let (base, client_id, client_secret, redirect, scopes, force_login) = registered.into_parts();
    ///
    /// assert_eq!(orig_base, &base);
    /// assert_eq!(orig_client_id, &client_id);
    /// assert_eq!(orig_client_secret, &client_secret);
    /// assert_eq!(orig_redirect, &redirect);
    /// assert_eq!(orig_scopes, scopes);
    /// assert_eq!(orig_force_login, force_login);
    /// ```
    pub fn into_parts(self) -> (String, String, String, String, Scopes, bool) {
        (
            self.base,
            self.client_id,
            self.client_secret,
            self.redirect,
            self.scopes,
            self.force_login,
        )
    }

    /// Returns the full url needed for authorization. This needs to be opened
    /// in a browser.
    pub fn authorize_url(&self) -> Result<String> {
        let scopes = format!("{}", self.scopes);
        let scopes: String = utf8_percent_encode(&scopes, NON_ALPHANUMERIC).collect();
        let url = if self.force_login {
            format!(
                "{}/oauth/authorize?client_id={}&redirect_uri={}&scope={}&force_login=true&\
                 response_type=code",
                self.base, self.client_id, self.redirect, scopes,
            )
        } else {
            format!(
                "{}/oauth/authorize?client_id={}&redirect_uri={}&scope={}&response_type=code",
                self.base, self.client_id, self.redirect, scopes,
            )
        };

        Ok(url)
    }

    /// Construct authentication data once token is known
    fn registered(&self, token: String) -> Data {
        Data {
            base: self.base.clone().into(),
            client_id: self.client_id.clone().into(),
            client_secret: self.client_secret.clone().into(),
            redirect: self.redirect.clone().into(),
            token: token.into(),
        }
    }

    /// Create an access token from the client id, client secret, and code
    /// provided by the authorization url.
    pub async fn complete<C>(&self, code: C) -> Result<Mastodon>
    where
        C: AsRef<str>,
    {
        let url =
            format!(
            "{}/oauth/token?client_id={}&client_secret={}&code={}&grant_type=authorization_code&\
             redirect_uri={}",
            self.base, self.client_id, self.client_secret, code.as_ref(), self.redirect
        );
        debug!(url = url; "completing registration");
        let response = self.client.post(&url).send().await?;
        debug!(
            status:serde = crate::helpers::log::Status::from(&response), url = url,
            headers:serde = crate::helpers::log::Headers::from(&response);
            "received API response"
        );
        let token: AccessToken = read_response(response).await?;
        debug!(url = url, body:serde = token; "parsed response body");
        let data = self.registered(token.access_token);
        trace!(auth_data:serde = data; "registered");

        Ok(Mastodon::new(self.client.clone(), data))
    }
}

/// Represents the state of the auth flow when the app has been registered but
/// the user is not authenticated
#[derive(Debug, Clone)]
pub struct Registered {
    base: String,
    client: Client,
    client_id: String,
    client_secret: String,
    redirect: String,
    scopes: Scopes,
    force_login: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registration_new() {
        let r = Registration::new("https://example.com");
        assert_eq!(r.base, "https://example.com".to_string());
        assert_eq!(r.app_builder, forms::ApplicationBuilder::default());
    }

    #[test]
    fn test_registration_with_sender() {
        let r = Registration::with_sender("https://example.com");
        assert_eq!(r.base, "https://example.com".to_string());
        assert_eq!(r.app_builder, forms::ApplicationBuilder::default());
    }

    #[test]
    fn test_set_client_name() {
        let mut r = Registration::new("https://example.com");
        r.client_name("foo-test");

        assert_eq!(r.base, "https://example.com".to_string());
        assert_eq!(
            &mut r.app_builder,
            forms::ApplicationBuilder::default().client_name("foo-test")
        );
    }

    #[test]
    fn test_set_redirect_uris() {
        let mut r = Registration::new("https://example.com");
        r.redirect_uris("https://foo.com");

        assert_eq!(r.base, "https://example.com".to_string());
        assert_eq!(
            &mut r.app_builder,
            forms::ApplicationBuilder::default().redirect_uris("https://foo.com")
        );
    }

    #[test]
    fn test_set_scopes() {
        let mut r = Registration::new("https://example.com");
        r.scopes(Scopes::all());

        assert_eq!(r.base, "https://example.com".to_string());
        assert_eq!(
            &mut r.app_builder,
            forms::ApplicationBuilder::default().scopes(Scopes::all())
        );
    }

    #[test]
    fn test_set_website() {
        let mut r = Registration::new("https://example.com");
        r.website("https://website.example.com");

        assert_eq!(r.base, "https://example.com".to_string());
        assert_eq!(
            &mut r.app_builder,
            forms::ApplicationBuilder::default().website("https://website.example.com")
        );
    }

    #[test]
    fn test_default_redirect_uri() {
        assert_eq!(&default_redirect_uri()[..], DEFAULT_REDIRECT_URI);
    }
}
