use paste::paste;
use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

/// ID that can be constructed from a string.
trait Id {
    fn new(value: impl Into<String>) -> Self;
}

/// Deserialization visitor for ID types.
///
/// Mastodon IDs are always strings, but Mastodon proper generates them from integer database keys,
/// and is not consistent about stringifying them. Small sequential IDs like rule and role IDs may
/// be serialized as numeric literals instead of strings, but should still be parsed into strings.
struct IdVisitor<T> {
    phantom: PhantomData<T>,
}

impl<T> IdVisitor<T> {
    fn new() -> Self {
        IdVisitor {
            phantom: PhantomData,
        }
    }
}

impl<'de, T: Id> Visitor<'de> for IdVisitor<T> {
    type Value = T;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an ID as a string or integer")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(T::new(v.to_string()))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(T::new(v.to_string()))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(T::new(v))
    }
}

macro_rules! define_ids {
    ($doc:literal as $name:ident, $($rest_doc:literal as $rest_name:ident,)+) => {
        define_ids!($doc as $name,);
        static_assertions::assert_not_impl_any!(
            $name: $(PartialEq<$rest_name>,)+
            PartialEq<String>,
        );
        define_ids!($($rest_doc as $rest_name,)+);
    };
    ($doc:literal as $name:ident,) => {
        paste! {
            #[doc = "Wrapper type for " $doc ]
            #[derive(Debug, Clone, Serialize, PartialEq, Eq)]
            #[serde(transparent)]
            pub struct $name(String);
        }

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

        impl Id for $name {
            fn new(value: impl Into<String>) -> Self {
                Self::new(value)
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<$name, D::Error> {
                deserializer.deserialize_any(IdVisitor::new())
            }
        }
    };
    () => {}
}

define_ids!(
    "an account ID" as AccountId,
    "an attachment ID" as AttachmentId,
    "a filter ID" as FilterId,
    "a filter keyword ID" as KeywordId,
    "the ID of an instance of a filtered status. See [`filter::Status`](crate::filter::Status)" as FilteredStatusId,
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
    "an ID of a domain allow rule" as DomainAllowId,
    "an ID of a domain block" as DomainBlockId,
    "an ID of an email domain block" as EmailDomainBlockId,
    "an ID of an IP range block" as IpBlockId,
    "a measurement key" as MeasureKey,
    "an announcement ID" as AnnouncementId,
    "a VAPID key for the push streaming API" as VapidKey,
    "a conversation ID" as ConversationId,
    "a poll ID" as PollId,
    "a hashtag ID" as TagId,
    "the ID of an application. As [`Application`](crate::application::Application) doesn't have an ID, I'm not sure what you're supposed to compare this to." as ApplicationId,
    "a role ID" as RoleId,
    "a warning preset ID. These are not yet exposed by the Mastodon API." as WarningPresetId,
);
