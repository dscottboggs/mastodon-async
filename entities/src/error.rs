use derive_is_enum_variant::is_enum_variant;

/// Error type
#[derive(Debug, thiserror::Error, is_enum_variant)]
pub enum Error {
    #[error("unrecognized visibility '{invalid}'")]
    VisibilityParsingError { invalid: String },
    #[error("unknown scope {0}")]
    UnknownScope(String),
    #[error(transparent)]
    Builder(#[from] derive_builder::UninitializedFieldError),
    #[error(transparent)]
    UrlEncodingError(serde_urlencoded::ser::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
