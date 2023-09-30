use std::collections::HashMap;
use std::string::FromUtf8Error;
use std::{error, fmt, io::Error as IoError, num::TryFromIntError};

use derive_builder::UninitializedFieldError;
use derive_is_enum_variant::is_enum_variant;
#[cfg(feature = "env")]
use envy::Error as EnvyError;
use reqwest::{header::ToStrError as HeaderStrError, Error as HttpError, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
#[cfg(feature = "toml")]
use tomlcrate::de::Error as TomlDeError;
#[cfg(feature = "toml")]
use tomlcrate::ser::Error as TomlSerError;
use url::ParseError as UrlError;

/// Convience type over `std::result::Result` with `Error` as the error type.
pub type Result<T> = ::std::result::Result<T, Error>;

/// enum of possible errors encountered using the mastodon API.
#[derive(Debug, thiserror::Error, is_enum_variant)]
pub enum Error {
    /// Error from the Mastodon API. This typically means something went
    /// wrong with your authentication or data.
    #[error("API error: status: {status:?}, response:\n{response:#?}")]
    Api {
        /// The response status.
        status: StatusCode,
        /// The JSON-decoded error response from the server.
        response: ApiError,
    },
    /// Error deserialising to json. Typically represents a breaking change in
    /// the Mastodon API
    #[error("error from serde")]
    Serde(#[from] SerdeError),
    /// Error serializing to (or from) url-encoded string
    #[error(transparent)]
    UrlEncoded(#[from] serde_qs::Error),
    /// Error encountered in the HTTP backend while requesting a route.
    #[error("Error encountered in the HTTP backend while requesting a route.")]
    Http(#[from] HttpError),
    /// Wrapper around the `std::io::Error` struct.
    #[error("io error")]
    Io(#[from] IoError),
    /// Wrapper around the `url::ParseError` struct.
    #[error("error parsing URL")]
    Url(#[from] UrlError),
    /// Missing Client Id.
    #[error("Missing Client Id.")]
    ClientIdRequired,
    /// Missing Client Secret.
    #[error("Missing Client Secret.")]
    ClientSecretRequired,
    /// Missing Access Token.
    #[error("Missing Access Token.")]
    AccessTokenRequired,
    /// Error serializing to toml
    #[cfg(feature = "toml")]
    #[error("Error serializing to toml")]
    TomlSer(#[from] TomlSerError),
    /// Error deserializing from toml
    #[cfg(feature = "toml")]
    #[error("Error deserializing from toml")]
    TomlDe(#[from] TomlDeError),

    /// Error raised in the helpers::json::to_writer or helpers::toml::to_writer function if not
    /// all bytes were written to the writer
    #[cfg(any(feature = "toml", feature = "json"))]
    #[error("Not all bytes were written")]
    NotAllBytesWritten,

    /// Error converting an http header to a string
    #[error("Error converting an http header to a string")]
    HeaderStrError(#[from] HeaderStrError),
    /// Error parsing the http Link header
    #[error("error parsing http link header")]
    LinkHeaderParse(#[from] parse_link_header::Error),
    /// Error returned when an unexpected rel was parsed.
    #[error("unrecognized rel {rel:?} in link header {link:?}")]
    UnrecognizedRel {
        /// The relation which was not recognized
        rel: String,
        /// The raw link header
        link: String,
    },
    #[cfg(feature = "env")]
    /// Error deserializing config from the environment
    #[error("Error deserializing config from the environment")]
    Envy(#[from] EnvyError),
    /// An integer conversion was attempted, but the value didn't fit into the
    /// target type.
    ///
    /// At the time of writing, this can only be triggered when a file is
    /// larger than the system's usize allows.
    #[error("integer didn't fit in the target size")]
    IntConversion(#[from] TryFromIntError),
    /// Error from mastodon-async-entities
    #[error(transparent)]
    Entities(#[from] mastodon_async_entities::error::Error),
    /// Error parsing UTF-8 string from bytes
    #[error(transparent)]
    FromUtf8(#[from] FromUtf8Error),
    /// Error constructing type from its builder
    #[error(transparent)]
    Builder(#[from] UninitializedFieldError),
    /// Other errors
    #[error("other error: {0:?}")]
    Other(String),
}

/// Error returned from the Mastodon API.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiError {
    /// The error message.
    pub error: String,
    /// A longer description of the error, mainly provided with the OAuth API.
    pub error_description: Option<String>,
    /// Details about the error. See this note in the docs https://docs.joinmastodon.org/methods/accounts/#422-unprocessable-entity
    pub details: Option<HashMap<String, Vec<ApiErrorDetail>>>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl error::Error for ApiError {}

#[macro_export]
/// Used to easily create errors from strings
macro_rules! format_err {
    ( $( $arg:tt )* ) => {
        {
            $crate::Error::Other(format!($($arg)*))
        }
    }
}

/// Details about the error. See this note in the docs https://docs.joinmastodon.org/methods/accounts/#422-unprocessable-entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiErrorDetail {
    error: ApiErrorDetailType,
    description: String,
}

/// The type of error detail.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum ApiErrorDetailType {
    /// When e-mail provider is not allowed
    #[serde(rename = "ERR_BLOCKED")]
    Blocked,
    /// When e-mail address does not resolve to any IP via DNS (MX, A, AAAA)
    #[serde(rename = "ERR_UNREACHABLE")]
    Unreachable,
    /// When username or e-mail are already taken
    #[serde(rename = "ERR_TAKEN")]
    Taken,
    /// When a username is reserved, e.g. “webmaster” or “admin”
    #[serde(rename = "ERR_RESERVED")]
    Reserved,
    /// When agreement has not been accepted
    #[serde(rename = "ERR_ACCEPTED")]
    Accepted,
    /// When a required attribute is blank
    #[serde(rename = "ERR_BLANK")]
    Blank,
    /// When an attribute is malformed, e.g. wrong characters or invalid e-mail address
    #[serde(rename = "ERR_INVALID")]
    Invalid,
    /// When an attribute is over the character limit
    #[serde(rename = "ERR_TOO_LONG")]
    TooLong,
    /// When an attribute is under the character requirement
    #[serde(rename = "ERR_TOO_SHORT")]
    TooShort,
    /// When an attribute is not one of the allowed values, e.g. unsupported locale:qa
    ///
    #[serde(rename = "ERR_INCLUSION")]
    Inclusion,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    macro_rules! assert_is {
        ($err:ident, $variant:pat) => {
            assert!(match $err {
                $variant => true,
                _ => false,
            });
        };
    }

    #[tokio::test]
    async fn from_http_error() {
        let err: HttpError = reqwest::get("not an actual URL").await.unwrap_err();
        let err: Error = Error::from(err);
        assert_is!(err, Error::Http(..));
    }

    #[test]
    fn from_io_error() {
        let err: IoError = io::Error::new(io::ErrorKind::Other, "other error");
        let err: Error = Error::from(err);
        assert_is!(err, Error::Io(..));
    }

    #[test]
    fn from_serde_error() {
        let err: SerdeError = serde_json::from_str::<()>("not valid json").unwrap_err();
        let err: Error = Error::from(err);
        assert_is!(err, Error::Serde(..));
    }

    #[test]
    fn from_url_encoded_error() {
        let err: serde_qs::Error = serde_qs::Error::Custom("error".into());
        let err: Error = Error::from(err);
        assert_is!(err, Error::UrlEncoded(..));
    }

    #[test]
    fn from_url_error() {
        let err: UrlError = UrlError::EmptyHost;
        let err: Error = Error::from(err);
        assert_is!(err, Error::Url(..));
    }

    #[cfg(feature = "toml")]
    #[test]
    fn from_toml_de_error() {
        use tomlcrate;
        let err: TomlDeError = tomlcrate::from_str::<()>("not valid toml").unwrap_err();
        let err: Error = Error::from(err);
        assert_is!(err, Error::TomlDe(..));
    }
}
