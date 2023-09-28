pub mod token;
pub use token::{TokenRequest, TokenRequestBuilder};

use derive_builder::Builder;
use derive_is_enum_variant::is_enum_variant;
use isolang::Language;
use serde::{Deserialize, Serialize};

use crate::{error::Result, prelude::Scopes, ClientId, Error};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, is_enum_variant, Default)]
#[serde(rename_all = "lowercase")]
/// The value for the `response_type` field of [`AuthorizationRequest`]. At the
/// time of writing, only the value `"code"` is valid; if other values become
/// valid they should be added to this enum.
pub enum AuthorizationResponseType {
    #[default]
    Code,
}

/// Form to be submitted by [`Mastodon::request_oauth_authorization`]
///
/// See also [the API reference](https://docs.joinmastodon.org/methods/oauth/#authorize)
#[derive(Clone, Builder, Debug, Serialize, Deserialize, PartialEq)]
#[builder(
    custom_constructor,
    derive(Debug, PartialEq),
    build_fn(error = "crate::Error", private, name = "try_build")
)]
pub struct AuthorizationRequest {
    /// Should be set equal to `"code"`.
    #[builder(default)]
    pub response_type: AuthorizationResponseType,
    /// The client ID, obtained during app registration.
    #[builder(private)]
    pub client_id: ClientId,
    /// Set a URI to redirect the user to. If this parameter is set to
    /// `"urn:ietf:wg:oauth:2.0:oob"` (the default) then the authorization code
    /// will be shown instead. Must match one of the `redirect_uris` declared
    /// during app registration.
    #[builder(setter(into), default = r#"String::from("urn:ietf:wg:oauth:2.0:oob")"#)]
    pub redirect_uri: String,
    /// List of requested OAuth scopes, separated by spaces (or by pluses, if
    /// using query parameters). Must be a subset of scopes declared during app
    /// registration. If not provided, defaults to read.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    pub scope: Option<Scopes>,
    /// Forces the user to re-login, which is necessary for authorizing with
    /// multiple accounts from the same instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    pub force_login: Option<bool>,
    /// The ISO 639-1 two-letter language code to use while rendering the authorization form.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    pub lang: Option<Language>,

    #[serde(skip)]
    #[builder(private)]
    pub instance: String,
}

impl AuthorizationRequest {
    pub fn builder(instance: String, client_id: ClientId) -> AuthorizationRequestBuilder {
        let mut it = AuthorizationRequestBuilder::create_empty();
        it.client_id(client_id).instance(instance);
        it
    }

    /// The URL the user needs to visit to authorize the application.
    pub fn url(&self) -> Result<String> {
        let base = &self.instance;
        match serde_urlencoded::to_string(self) {
            Ok(query) => Ok(format!("{base}?{query}")),
            Err(error) => Err(Error::UrlEncodingError(error)),
        }
    }
}

impl AuthorizationRequestBuilder {
    pub fn build(&self) -> AuthorizationRequest {
        self.try_build()
            .expect("One or more required fields are missing!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let mut builder =
            AuthorizationRequest::builder("https://tams.tech".into(), ClientId::new("client_id"));
        let req = builder.build();
        assert!(req.response_type.is_code());
        assert_eq!(req.client_id.as_ref(), "client_id");
        assert_eq!(req.redirect_uri, "urn:ietf:wg:oauth:2.0:oob");
        assert!(req.scope.is_none());
        assert!(req.force_login.is_none());
        assert!(req.lang.is_none());
        let esperanto = Language::from_639_1("eo").unwrap();
        builder
            .redirect_uri("redirect_uri")
            .scope(Scopes::read_all())
            .force_login(true)
            .lang(esperanto);
        let req = builder.build();
        assert!(req.response_type.is_code()); // can't be anything else as of writing
        assert_eq!(req.client_id.as_ref(), "client_id");
        assert_eq!(req.redirect_uri, "redirect_uri");
        assert_eq!(Some(Scopes::read_all()), req.scope);
        assert_eq!(req.force_login, Some(true));
        assert_eq!(req.lang, Some(esperanto));
    }
}
