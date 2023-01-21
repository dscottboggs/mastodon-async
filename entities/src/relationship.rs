//! module containing everything relating to a relationship with
//! another account.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// A struct containing information about a relationship with another account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Relationship {
    /// Target account id
    pub id: RelationshipId,
    /// Whether the application client follows the account.
    pub following: bool,
    /// Whether the account follows the application client.
    pub followed_by: bool,
    /// Whether the application client blocks the account.
    pub blocking: bool,
    /// Whether the application client blocks the account.
    pub muting: bool,
    /// Whether the application client has requested to follow the account.
    pub requested: bool,
    /// Whether the user is also muting notifications
    pub muting_notifications: bool,
    /// Whether the user is currently blocking the accounts's domain
    pub domain_blocking: bool,
    /// Whether the user's reblogs will show up in the home timeline
    pub showing_reblogs: bool,
    /// Whether the user is currently endorsing the account
    ///
    /// This field is not techincally nullable with mastodon >= 2.5.0, but
    /// making it `Option<bool>` here means we shouldn't get deser errors when
    /// making calls to pleroma or mastodon<2.5.0 instances
    pub endorsed: Option<bool>,
}

/// Wrapper type for a relationship ID string
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct RelationshipId(String);

impl AsRef<str> for RelationshipId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl RelationshipId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}
impl Display for RelationshipId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

static_assertions::assert_not_impl_any!(
    RelationshipId: PartialEq<crate::account::AccountId>,
    PartialEq<crate::attachment::AttachmentId>,
    PartialEq<crate::filter::FilterId>,
    PartialEq<crate::push::SubscriptionId>,
    PartialEq<crate::mention::MentionId>,
    PartialEq<crate::notification::NotificationId>,
    PartialEq<crate::list::ListId>,
    PartialEq<crate::report::ReportId>,
    PartialEq<crate::status::StatusId>,
);
