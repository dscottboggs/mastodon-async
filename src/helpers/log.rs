use serde::{Deserialize, Serialize};

/// Serializable form of reqwest's Status type.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Status {
    /// The numerical representation of the status
    pub code: u16,
    /// it's canonical reason.
    pub message: Option<&'static str>,
}

impl Status {
    /// New from reqwest's Status type (which is more useful but not
    /// serializable).
    pub fn new(status: reqwest::StatusCode) -> Self {
        Self {
            code: status.as_u16(),
            message: status.canonical_reason(),
        }
    }
}

impl From<&reqwest::Response> for Status {
    fn from(value: &reqwest::Response) -> Self {
        Self::new(value.status())
    }
}

/// Helper for logging request headers
#[derive(Debug)]
pub struct Headers<'h>(pub &'h reqwest::header::HeaderMap);

impl<'h> Serialize for Headers<'h> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_map(
            self.0
                .iter()
                .map(|(k, v)| (format!("{k:?}"), format!("{v:?}"))),
        )
    }
}

impl<'h> From<&'h reqwest::Response> for Headers<'h> {
    fn from(value: &'h reqwest::Response) -> Self {
        Headers(value.headers())
    }
}
