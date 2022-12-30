/// Error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unrecognized visibility '{invalid}'")]
    VisibilityParsingError { invalid: String },
}
