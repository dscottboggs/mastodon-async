use derive_builder::Builder;
use serde::Serialize;
// use try_from::TryInto;

use crate::scopes::Scopes;

/// Represents an application that can be registered with a mastodon instance
#[derive(Clone, Builder, Debug, Default, Serialize, PartialEq)]
#[builder(derive(Debug, PartialEq), build_fn(error = "crate::Error"))]
pub struct App {
    /// The name the client will identify itself with
    #[builder(setter(into))]
    client_name: String,
    /// Where the user should be redirected after authorization. To display the
    /// authorization code to the user instead of redirecting to a web page, use
    /// `"urn:ietf:wg:oauth:2.0:oob"` in this parameter.
    #[builder(setter(into), default = r#""urn:ietf:wg:oauth:2.0:oob".into()"#)]
    redirect_uris: String,
    /// Scopes the application is requesting access to.
    #[builder(default = "Scopes::read_all()")]
    scopes: Scopes,
    /// A URL to the homepage of your app
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    website: Option<String>,
}

impl App {
    /// Get an AppBuilder object
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::apps::App;
    ///
    /// let mut builder = App::builder();
    /// ```
    pub fn builder() -> AppBuilder {
        AppBuilder::default()
    }

    /// Retrieve the list of scopes that apply to this App
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::{apps::App, scopes::Scopes};
    ///
    /// let mut builder = App::builder();
    /// builder.client_name("mastodon-async-test");
    /// let app = builder.build().unwrap();
    /// let scopes = app.scopes();
    /// assert_eq!(scopes, &Scopes::read_all());
    /// ```
    pub fn scopes(&self) -> &Scopes {
        &self.scopes
    }
}

impl TryFrom<AppBuilder> for App {
    type Error = crate::Error;

    fn try_from(value: AppBuilder) -> Result<Self, Self::Error> {
        value.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_builder() {
        let builder = App::builder();
        assert_eq!(builder, AppBuilder::default());
    }

    #[test]
    fn test_app_scopes() {
        let mut builder = App::builder();
        builder.client_name("test").scopes(Scopes::all());
        let app = builder.build().expect("Couldn't build App");
        assert_eq!(app.scopes(), &Scopes::all());
    }

    #[test]
    fn test_app_builder_all_methods() {
        let mut builder = AppBuilder::default();
        builder.client_name("foo-test");
        builder.redirect_uris("http://example.com");
        builder.scopes(Scopes::read_all() | Scopes::write_all());
        builder.website("https://example.com");
        let app = builder.build().expect("Couldn't build App");
        assert_eq!(
            app,
            App {
                client_name: "foo-test".to_string(),
                redirect_uris: "http://example.com".to_string(),
                scopes: Scopes::read_all() | Scopes::write_all(),
                website: Some("https://example.com".to_string()),
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_app_builder_build_fails_if_no_client_name_1() {
        App::builder().build().expect("no client-name");
    }

    #[test]
    #[should_panic]
    fn test_app_builder_build_fails_if_no_client_name_2() {
        let mut builder = App::builder();
        builder
            .website("https://example.com")
            .redirect_uris("https://example.com")
            .scopes(Scopes::all());
        builder.build().expect("no client-name");
    }

    #[test]
    fn test_app_try_into_app() {
        let app = App {
            client_name: "foo-test".to_string(),
            redirect_uris: "http://example.com".to_string(),
            scopes: Scopes::all(),
            website: None,
        };
        let expected = app.clone();
        #[allow(clippy::useless_conversion)]
        let result = app.try_into().expect("Couldn't make App into App");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_app_builder_try_into_app() {
        let mut builder = App::builder();
        builder
            .client_name("foo-test")
            .redirect_uris("http://example.com")
            .scopes(Scopes::all());
        let expected = App {
            client_name: "foo-test".to_string(),
            redirect_uris: "http://example.com".to_string(),
            scopes: Scopes::all(),
            website: None,
        };
        let result = builder
            .try_into()
            .expect("Couldn't make AppBuilder into App");
        assert_eq!(expected, result);
    }
}
