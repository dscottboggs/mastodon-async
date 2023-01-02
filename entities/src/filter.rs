use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// Represents a single Filter
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Filter {
    id: String,
    phrase: String,
    context: Vec<FilterContext>,
    #[serde(with = "iso8601::option")]
    expires_at: Option<OffsetDateTime>,
    irreversible: bool,
    whole_word: bool,
}

/// Represents the various types of Filter contexts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterContext {
    /// Represents the "home" context
    #[serde(rename = "home")]
    Home,
    /// Represents the "notifications" context
    #[serde(rename = "notifications")]
    Notifications,
    /// Represents the "public" context
    #[serde(rename = "public")]
    Public,
    /// Represents the "thread" context
    #[serde(rename = "thread")]
    Thread,
}
