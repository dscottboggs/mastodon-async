use derive_builder::Builder;
use mastodon_async_entities::{ClientId, ClientSecret, OAuthToken};
use serde::{Deserialize, Serialize};

/// Raw data about mastodon app. Save `Data` using `serde` to prevent needing
/// to authenticate on every run.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Builder)]
#[builder(
    custom_constructor,
    derive(Debug, PartialEq),
    build_fn(error = "crate::Error", private, name = "try_build")
)]
pub struct Data {
    #[builder(private, setter(into))]
    /// Base url of instance eg. `https://botsin.space`.
    pub base: String,
    #[builder(private, setter(into))]
    /// The client's id given by the instance.
    pub client_id: ClientId,
    #[builder(private, setter(into))]
    /// The client's secret given by the instance.
    pub client_secret: ClientSecret,
    #[builder(setter(into), default = r#""urn:ietf:wg:oauth:2.0:oob".into()"#)]
    /// Url to redirect back to your application from the instance signup.
    pub redirect: String,
    #[builder(private, setter(into))]
    /// The client's access token.
    pub token: OAuthToken,
}

impl Data {
    /// Return a builder for `Data`, providing all required fields.
    pub fn builder(
        base: impl Into<String>,
        client_id: ClientId,
        client_secret: ClientSecret,
        token: OAuthToken,
    ) -> DataBuilder {
        let mut builder = DataBuilder::create_empty();
        builder
            .base(base)
            .client_id(client_id)
            .client_secret(client_secret)
            .token(token);
        builder
    }
}

impl DataBuilder {
    /// Return the constructed Data object
    pub fn build(&self) -> Data {
        self.try_build()
            .expect("One or more required fields are missing!")
    }
}

impl Default for Data {
    /// Only for demonstration purposes and to shorten documentation. Not useful!
    fn default() -> Self {
        Self {
            client_id: String::default().into(),
            client_secret: String::default().into(),
            base: String::default(),
            redirect: String::default(),
            token: String::default().into(),
        }
    }
}
