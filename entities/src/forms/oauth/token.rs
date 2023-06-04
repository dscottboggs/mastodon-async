use derive_builder::Builder;
use derive_is_enum_variant::is_enum_variant;
use serde::{Deserialize, Serialize};

use crate::prelude::{ClientId, ClientSecret, Scopes};

/// The value for the [`TokenRequest`] `grant_type` field.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, is_enum_variant, Default)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
    #[default]
    ClientCredentials,
}

#[derive(Clone, Builder, Debug, Serialize, Deserialize, PartialEq)]
#[builder(
    custom_constructor,
    derive(Debug, PartialEq),
    build_fn(error = "crate::Error", private, name = "try_build")
)]
pub struct TokenRequest {
    /// Set equal to [`GrantType::AuthorizationCode`] if code is provided in
    /// order to gain user-level access. Otherwise, set equal to
    /// [`GrantType::ClientCredentials`] to obtain app-level access only.
    #[builder(default)]
    grant_type: GrantType,
    /// A user authorization code, obtained via
    /// [`Mastodon::request_oauth_authorization()`](https://docs.rs/mastodon-async/latest/mastodon_async/mastodon/struct.Mastodon.html#method.request_oauth_authorization)
    #[builder(setter(into, strip_option))]
    code: Option<String>,
    /// The client ID, obtained during app registration.
    #[builder(private)]
    client_id: ClientId,
    /// The client secret, obtained during app registration.
    #[builder(private)]
    client_secret: ClientSecret,
    /// Set a URI to redirect the user to. If this parameter is set to
    /// `"urn:ietf:wg:oauth:2.0:oob"` then the token will be shown instead. Must
    /// match one of the `redirect_uri`s declared during app registration.
    #[builder(default = r#""urn:ietf:wg:oauth:2.0:oob".into()"#)]
    redirect_uri: String,
    /// List of requested OAuth scopes. If code was provided, then this must be
    /// equal to the scope requested from the user. Otherwise, it must be a
    /// subset of scopes declared during app registration. If not provided,
    /// defaults to [`Scope::Read(None)`](crate::prelude::Scope::Read).
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<Scopes>,
}

impl TokenRequest {
    pub fn builder(client_id: ClientId, client_secret: ClientSecret) -> TokenRequestBuilder {
        TokenRequestBuilder::create_empty()
            .client_id(client_id)
            .client_secret(client_secret)
            .to_owned()
    }
}

impl TokenRequestBuilder {
    pub fn build(&self) -> TokenRequest {
        self.try_build()
            .expect("One or more required fields are missing!")
    }
}
