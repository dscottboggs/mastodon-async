//! module containing information about a finished report of a user.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// A struct containing info about a report.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Report {
    /// The ID of the report.
    pub id: ReportId,
    /// The action taken in response to the report.
    pub action_taken: String,
}

/// Wrapper type for a report ID string
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct ReportId(String);

impl AsRef<str> for ReportId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ReportId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl Display for ReportId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

static_assertions::assert_not_impl_any!(
    ReportId: PartialEq<crate::account::AccountId>,
    PartialEq<crate::attachment::AttachmentId>,
    PartialEq<crate::filter::FilterId>,
    PartialEq<crate::push::SubscriptionId>,
    PartialEq<crate::mention::MentionId>,
    PartialEq<crate::notification::NotificationId>,
    PartialEq<crate::relationship::RelationshipId>,
    PartialEq<crate::list::ListId>,
    PartialEq<crate::status::StatusId>,
);
