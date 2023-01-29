use serde::{Deserialize, Serialize};
use std::fmt::Display;

macro_rules! define_ids {
    ($name:ident, $($rest:ident,)+) => {
        define_ids!($name,);
        static_assertions::assert_not_impl_any!(
            $name: $(PartialEq<$rest>,)+
        );
        define_ids!($($rest,)+);
    };
    ($name:ident,) => {
        /// Wrapper type for a account ID string
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
    AccountId,
    AttachmentId,
    FilterId,
    ListId,
    MentionId,
    NotificationId,
    SubscriptionId,
    RelationshipId,
    ReportId,
    StatusId,
    RuleId,
    CanonicalEmailBlockId,
    DimensionKey,
    DimensionDataKey,
    DomainId,
);
