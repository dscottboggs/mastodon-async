use is_variant::IsVariant;

/// Error type
#[derive(Debug, thiserror::Error, IsVariant)]
pub enum Error {
    #[error("unrecognized visibility '{invalid}'")]
    VisibilityParsingError { invalid: String },
    #[error("unknown scope {0}")]
    UnknownScope(String),
    #[error(transparent)]
    Builder(#[from] derive_builder::UninitializedFieldError),
}
