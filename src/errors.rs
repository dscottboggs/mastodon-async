use std::{error, fmt, io::Error as IoError, num::TryFromIntError};

#[cfg(feature = "env")]
use envy::Error as EnvyError;
use hyper_old_types::Error as HeaderParseError;
#[cfg(feature = "magic")]
use magic::MagicError;
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
#[derive(Debug)]
pub enum Error {
    /// Error from the Mastodon API. This typically means something went
    /// wrong with your authentication or data.
    Api {
        /// The response status.
        status: StatusCode,
        /// The JSON-decoded error response from the server.
        response: ApiError,
    },
    /// Error deserialising to json. Typically represents a breaking change in
    /// the Mastodon API
    Serde(SerdeError),
    /// Error serializing to url-encoded string
    UrlEncoded(UrlEncodedError),
    /// Error encountered in the HTTP backend while requesting a route.
    Http(HttpError),
    /// Wrapper around the `std::io::Error` struct.
    Io(IoError),
    /// Wrapper around the `url::ParseError` struct.
    Url(UrlError),
    /// Missing Client Id.
    ClientIdRequired,
    /// Missing Client Secret.
    ClientSecretRequired,
    /// Missing Access Token.
    AccessTokenRequired,
    /// MastodonBuilder & AppBuilder error
    MissingField(&'static str),
    #[cfg(feature = "toml")]
    /// Error serializing to toml
    TomlSer(TomlSerError),
    #[cfg(feature = "toml")]
    /// Error deserializing from toml
    TomlDe(TomlDeError),
    /// Error converting an http header to a string
    HeaderStrError(HeaderStrError),
    /// Error parsing the http Link header
    HeaderParseError(HeaderParseError),
    #[cfg(feature = "env")]
    /// Error deserializing from the environment
    Envy(EnvyError),
    /// Error serializing to a query string
    SerdeQs(SerdeQsError),
    /// An integer conversion was attempted, but the value didn't fit into the
    /// target type.
    ///
    /// At the time of writing, this can only be triggered when a file is
    /// larger than the system's usize allows.
    IntConversion(TryFromIntError),
    #[cfg(feature = "magic")]
    /// An error received from the magic crate
    Magic(MagicError),
    /// Other errors
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match *self {
            Serde(ref e) => Some(e),
            UrlEncoded(ref e) => Some(e),
            Http(ref e) => Some(e),
            Io(ref e) => Some(e),
            Url(ref e) => Some(e),
            #[cfg(feature = "toml")]
            TomlSer(ref e) => Some(e),
            #[cfg(feature = "toml")]
            TomlDe(ref e) => Some(e),
            HeaderStrError(ref e) => Some(e),
            HeaderParseError(ref e) => Some(e),
            #[cfg(feature = "env")]
            Envy(ref e) => Some(e),
            SerdeQs(ref e) => Some(e),
            IntConversion(ref e) => Some(e),
            #[cfg(feature = "magic")]
            Magic(ref e) => Some(e),
            Api {
                ..
            }
            | ClientIdRequired
            | ClientSecretRequired
            | AccessTokenRequired
            | MissingField(_)
            | Other(..) => None,
        }
    }
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

macro_rules! from {
    ($($(#[$met:meta])* $typ:ident => $variant:ident,)*) => {
        $(
            $(#[$met])*
            impl From<$typ> for Error {
                fn from(from: $typ) -> Self {
                    use Error::*;
                    $variant(from)
                }
            }
        )*
    }
}

from! {
    HttpError => Http,
    IoError => Io,
    SerdeError => Serde,
    UrlEncodedError => UrlEncoded,
    UrlError => Url,
    #[cfg(feature = "toml")]
    TomlSerError => TomlSer,
    #[cfg(feature = "toml")]
    TomlDeError => TomlDe,
    HeaderStrError => HeaderStrError,
    HeaderParseError => HeaderParseError,
    #[cfg(feature = "env")]
    EnvyError => Envy,
    SerdeQsError => SerdeQs,
    String => Other,
    TryFromIntError => IntConversion,
    #[cfg(feature = "magic")]
    MagicError => Magic,
}

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
