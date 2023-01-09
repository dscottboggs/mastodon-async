use serde::{Deserialize, Serialize};

/// Used for ser/de of list resources
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct List {
    id: ListId,
    title: String,
}

/// Wrapper type for a list ID string
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct ListId(String);

impl AsRef<str> for ListId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ListId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

static_assertions::assert_not_impl_any!(
    ListId: PartialEq<crate::account::AccountId>,
    PartialEq<crate::attachment::AttachmentId>,
    PartialEq<crate::filter::FilterId>,
    PartialEq<crate::push::SubscriptionId>,
    PartialEq<crate::mention::MentionId>,
    PartialEq<crate::notification::NotificationId>,
    PartialEq<crate::relationship::RelationshipId>,
    PartialEq<crate::report::ReportId>,
    PartialEq<crate::status::StatusId>,
);
