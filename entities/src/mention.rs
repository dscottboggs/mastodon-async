use serde::{Deserialize, Serialize};

/// Represents a `mention` used in a status
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Mention {
    /// URL of user's profile (can be remote)
    pub url: String,
    /// The username of the account
    pub username: String,
    /// Equals username for local users, includes `@domain` for remote ones
    pub acct: String,
    /// Account ID
    pub id: String,
}

/// Wrapper type for a mention ID string
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct MentionId(String);

impl AsRef<str> for MentionId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl MentionId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}
