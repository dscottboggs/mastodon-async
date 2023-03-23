use crate::auth;
use derive_builder::Builder;
use serde::Serialize;

/// Represents an application that can be registered with a mastodon instance
#[derive(Clone, Builder, Debug, Default, Serialize, PartialEq)]
#[builder(derive(Debug, PartialEq), build_fn(error = "crate::Error"))]
pub struct Application {
    /// The name the client will identify itself with
    #[builder(setter(into))]
    client_name: String,
    /// Where the user should be redirected after authorization. To display the
    /// authorization code to the user instead of redirecting to a web page, use
    /// `"urn:ietf:wg:oauth:2.0:oob"` in this parameter.
    #[builder(setter(into), default = r#""urn:ietf:wg:oauth:2.0:oob".into()"#)]
    redirect_uris: String,
    /// Scopes the application is requesting access to.
    #[builder(default = "auth::Scopes::read_all()")]
    scopes: auth::Scopes,
    /// A URL to the homepage of your app
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    website: Option<String>,
}

impl Application {
    /// Get an ApplicationBuilder object
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::apps::App;
    ///
    /// let mut builder = App::builder();
    /// ```
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }

    /// Retrieve the list of scopes that apply to this App
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::{apps::App, prelude::*};
    ///
    /// let mut builder = App::builder();
    /// builder.client_name("mastodon-async-test");
    /// let app = builder.build().unwrap();
    /// let scopes = app.scopes();
    /// assert_eq!(scopes, &auth::Scopes::read_all());
    /// ```
    pub fn scopes(&self) -> &auth::Scopes {
        &self.scopes
    }
}

impl TryFrom<ApplicationBuilder> for Application {
    type Error = crate::Error;

    fn try_from(value: ApplicationBuilder) -> Result<Self, Self::Error> {
        value.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_builder() {
        let builder = Application::builder();
        assert_eq!(builder, ApplicationBuilder::default());
    }

    #[test]
    fn test_app_scopes() {
        let mut builder = Application::builder();
        builder.client_name("test").scopes(auth::Scopes::all());
        let app = builder.build().expect("Couldn't build App");
        assert_eq!(app.scopes(), &auth::Scopes::all());
    }

    #[test]
    fn test_app_builder_all_methods() {
        let mut builder = ApplicationBuilder::default();
        builder.client_name("foo-test");
        builder.redirect_uris("http://example.com");
        builder.scopes(auth::Scopes::read_all() | auth::Scopes::write_all());
        builder.website("https://example.com");
        let app = builder.build().expect("Couldn't build App");
        assert_eq!(
            app,
            Application {
                client_name: "foo-test".to_string(),
                redirect_uris: "http://example.com".to_string(),
                scopes: auth::Scopes::read_all() | auth::Scopes::write_all(),
                website: Some("https://example.com".to_string()),
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_app_builder_build_fails_if_no_client_name_1() {
        Application::builder().build().expect("no client-name");
    }

    #[test]
    #[should_panic]
    fn test_app_builder_build_fails_if_no_client_name_2() {
        let mut builder = Application::builder();
        builder
            .website("https://example.com")
            .redirect_uris("https://example.com")
            .scopes(auth::Scopes::all());
        builder.build().expect("no client-name");
    }

    #[test]
    fn test_app_try_into_app() {
        let app = Application {
            client_name: "foo-test".to_string(),
            redirect_uris: "http://example.com".to_string(),
            scopes: auth::Scopes::all(),
            website: None,
        };
        let expected = app.clone();
        #[allow(clippy::useless_conversion)]
        let result = app.try_into().expect("Couldn't make App into App");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_app_builder_try_into_app() {
        let mut builder = Application::builder();
        builder
            .client_name("foo-test")
            .redirect_uris("http://example.com")
            .scopes(auth::Scopes::all());
        let expected = Application {
            client_name: "foo-test".to_string(),
            redirect_uris: "http://example.com".to_string(),
            scopes: auth::Scopes::all(),
            website: None,
        };
        let result = builder
            .try_into()
            .expect("Couldn't make ApplicationBuilder into App");
        assert_eq!(expected, result);
    }
}
