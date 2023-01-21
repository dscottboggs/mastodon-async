use std::fmt::Display;

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

impl Display for MentionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

static_assertions::assert_not_impl_any!(
    Mention: PartialEq<crate::account::AccountId>,
    PartialEq<crate::attachment::AttachmentId>,
    PartialEq<crate::filter::FilterId>,
    PartialEq<crate::list::ListId>,
    PartialEq<crate::notification::NotificationId>,
    PartialEq<crate::relationship::RelationshipId>,
    PartialEq<crate::push::SubscriptionId>,
    PartialEq<crate::report::ReportId>,
    PartialEq<crate::status::StatusId>,
    PartialEq<crate::instance::RuleId>,
);
