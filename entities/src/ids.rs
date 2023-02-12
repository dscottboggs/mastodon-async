use serde::{Deserialize, Serialize};
use std::fmt::Display;

macro_rules! define_ids {
    ($doc:literal as $name:ident(from $from_t:ty, as $ref_t:ident ref), $($rest_doc:literal as $rest_name:ident(from $rest_from_t:ty, as $rest_ref_t:ident ref),)+) => {
        define_ids!($doc as $name(from $from_t, as $ref_t ref),);
        static_assertions::assert_not_impl_any!(
            $name: $(PartialEq<$rest_name>,)+
            PartialEq<$from_t>,
        );
        define_ids!($($rest_doc as $rest_name(from $rest_from_t, as $rest_ref_t ref),)+);
    };
    ($doc:literal as $name:ident(from $from_t:ty, as $ref_t:ident ref),) => {
        /// Wrapper type for a account ID string
        #[doc = concat!("Wrapper type for ", $doc)]
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
        #[serde(transparent)]
        pub struct $name($from_t);

        impl AsRef<$ref_t> for $name {
            fn as_ref(&self) -> &$ref_t {
                &self.0
            }
        }

        impl $name {
            pub fn new(value: impl Into<$from_t>) -> Self {
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
    "an account ID" as AccountId(from String, as str ref),
    "an attachment ID" as AttachmentId(from String, as str ref),
    "a filter ID" as FilterId(from String, as str ref),
    "a filter keyword ID" as KeywordId(from String, as str ref),
    "the ID of an instance of a filtered status. See [`filter::Status`]" as FilteredStatusId(from String, as str ref),
    "a list ID" as ListId(from String, as str ref),
    "a mention ID" as MentionId(from String, as str ref),
    "a notification ID" as NotificationId(from String, as str ref),
    "a subscription ID" as SubscriptionId(from String, as str ref),
    "a relationship ID" as RelationshipId(from String, as str ref),
    "a report ID" as ReportId(from String, as str ref),
    "a status ID" as StatusId(from String, as str ref),
    "a rule ID" as RuleId(from String, as str ref),
    "a canonical email block ID" as CanonicalEmailBlockId(from String, as str ref),
    "a dimension key" as DimensionKey(from String, as str ref),
    "a dimension data element key" as DimensionDataKey(from String, as str ref),
    "an ID of a domain allow rule" as AllowDomainId(from String, as str ref),
    "an ID of a domain block" as DomainBlockId(from String, as str ref),
    "an ID of an email domain block" as EmailDomainBlockId(from String, as str ref),
    "a measurement key" as MeasureKey(from String, as str ref),
    "an announcement ID" as AnnouncementId(from String, as str ref),
    "a Vapid key for push streaming API" as VapidKey(from String, as str ref),
    "a conversation ID" as ConversationId(from String, as str ref),
    "a poll ID" as PollId(from String, as str ref),
    "a hashtag ID" as TagId(from String, as str ref),
    "the ID of an application.

As [`Application`] doesn't have an ID, I'm not sure what you're supposed to compare this to." as ApplicationId(from i64, as i64 ref),
    "a role ID" as RoleId(from i64, as i64 ref),
);
