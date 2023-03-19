use derive_is_enum_variant::is_enum_variant;

/// Error type
#[derive(Debug, thiserror::Error, is_enum_variant)]
pub enum Error {
    #[error("unrecognized visibility '{invalid}'")]
    VisibilityParsingError { invalid: String },
}
