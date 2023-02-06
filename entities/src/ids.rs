use serde::{Deserialize, Serialize};
use std::fmt::Display;

macro_rules! define_ids {
    ($doc:literal as $name:ident, $($rest_doc:literal as $rest_name:ident,)+) => {
        define_ids!($doc as $name,);
        static_assertions::assert_not_impl_any!(
            $name: $(PartialEq<$rest_name>,)+
        );
        define_ids!($($rest_doc as $rest_name,)+);
    };
    ($doc:literal as $name:ident,) => {
        /// Wrapper type for a account ID string
        #[doc = concat!("Wrapper type for ", $doc)]
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
        #[serde(transparent)]
        pub struct $name(String);

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
    () => {}
}

define_ids!(
    "an account ID" as AccountId,
    "an attachment ID" as AttachmentId,
    "a filter ID" as FilterId,
    "a list ID" as ListId,
    "a mention ID" as MentionId,
    "a notification ID" as NotificationId,
    "a subscription ID" as SubscriptionId,
    "a relationship ID" as RelationshipId,
    "a report ID" as ReportId,
    "a status ID" as StatusId,
    "a rule ID" as RuleId,
    "a canonical email block ID" as CanonicalEmailBlockId,
    "a dimension key" as DimensionKey,
    "a dimension data element key" as DimensionDataKey,
    "an ID of a domain allow rule" as AllowDomainId,
    "an ID of a domain block" as DomainBlockId,
    "an ID of an email domain block" as EmailDomainBlockId,
    "a measurement key" as MeasureKey,
    "an announcement ID" as AnnouncementId,
    "a Vapid key for push streaming API" as VapidKey,
    "a conversation ID" as ConversationId,
    "a poll ID" as PollId,
    "a hashtag ID" as TagId,
);

/// the ID of an application.
///
/// As [`Application`] doesn't have an ID, I'm not sure what you're supposed to compare this to.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct ApplicationId(i64);

impl AsRef<i64> for ApplicationId {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl ApplicationId {
    pub fn new(v: i64) -> Self {
        Self(v)
    }
}

impl Display for ApplicationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
