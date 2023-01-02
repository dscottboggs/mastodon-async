use serde::{Deserialize, Serialize};

mod v1 {
    pub use super::FilterContext;
    use serde::{Deserialize, Serialize};
    use time::{serde::iso8601, OffsetDateTime};

    /// Represents a single Filter
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Filter {
        /// The ID of the Filter in the database.
        pub id: String,
        /// The text to be filtered.
        pub phrase: String,
        /// The contexts in which the filter should be applied.
        pub context: Vec<FilterContext>,
        /// When the filter should no longer be applied.
        ///
        /// `None` indicates that the filter does not expire.
        #[serde(with = "iso8601::option")]
        pub expires_at: Option<OffsetDateTime>,
        /// Should matching entities in home and notifications be dropped by the server?
        pub irreversible: bool,
        /// Should the filter consider word boundaries?
        pub whole_word: bool,
    }
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
