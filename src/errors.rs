use std::{error, fmt, io::Error as IoError, num::TryFromIntError};

#[cfg(feature = "env")]
use envy::Error as EnvyError;
use hyper_old_types::Error as HeaderParseError;
use reqwest::{header::ToStrError as HeaderStrError, Error as HttpError, StatusCode};
use serde::Deserialize;
use serde_json::Error as SerdeError;
use serde_qs::Error as SerdeQsError;
use serde_urlencoded::ser::Error as UrlEncodedError;
#[cfg(feature = "toml")]
use tomlcrate::de::Error as TomlDeError;
#[cfg(feature = "toml")]
use tomlcrate::ser::Error as TomlSerError;
use url::ParseError as UrlError;

/// Convience type over `std::result::Result` with `Error` as the error type.
pub type Result<T> = ::std::result::Result<T, Error>;

/// enum of possible errors encountered using the mastodon API.
#[derive(Debug, thiserror::Error)]
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
    /// Error serializing to url-encoded string
    #[error("error serializing to url-encoded string")]
    UrlEncoded(#[from] UrlEncodedError),
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
    /// MastodonBuilder & AppBuilder error
    #[error("builder required field {0:?} to be constructed")]
    MissingField(&'static str),
    #[cfg(feature = "toml")]
    /// Error serializing to toml
    #[error("Error serializing to toml")]
    TomlSer(#[from] TomlSerError),
    #[cfg(feature = "toml")]
    /// Error deserializing from toml
    #[error("Error deserializing from toml")]
    TomlDe(#[from] TomlDeError),
    /// Error converting an http header to a string
    #[error("Error converting an http header to a string")]
    HeaderStrError(#[from] HeaderStrError),
    /// Error parsing the http Link header
    #[error("Error parsing the http Link header")]
    HeaderParseError(#[from] HeaderParseError),
    #[cfg(feature = "env")]
    /// Error deserializing config from the environment
    #[error("Error deserializing config from the environment")]
    Envy(#[from] EnvyError),
    /// Error serializing to a query string
    #[error("Error serializing to a query string")]
    SerdeQs(#[from] SerdeQsError),
    /// An integer conversion was attempted, but the value didn't fit into the
    /// target type.
    ///
    /// At the time of writing, this can only be triggered when a file is
    /// larger than the system's usize allows.
    #[error("integer didn't fit in the target size")]
    IntConversion(#[from] TryFromIntError),
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
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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
        let err: UrlEncodedError = serde_urlencoded::ser::Error::Custom("error".into());
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
    fn from_toml_ser_error() {
        let err: TomlSerError = TomlSerError::DateInvalid;
        let err: Error = Error::from(err);
        assert_is!(err, Error::TomlSer(..));
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
