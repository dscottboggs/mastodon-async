use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// Represents a user-defined filter for determining which statuses should not
/// be shown to the user.
///
/// ## Example
/// ```rust
/// use mastodon_async::entities::filter::Filter;
/// let subject = r#"{
///     "id": "19972",
///     "title": "Test filter",
///     "context": [
///         "home"
///     ],
///     "expires_at": "2022-09-20T17:27:39.296Z",
///     "filter_action": "warn",
///     "keywords": [
///         {
///             "id": "1197",
///             "keyword": "bad word",
///             "whole_word": false
///         }
///     ],
///     "statuses": [
///         {
///             "id": "1",
///             "status_id": "109031743575371913"
///         }
///     ]
/// }"#;
/// let subject: Filter = serde_json::from_str(subject).expect("deserialize");
/// assert_eq!(subject.id, "19972");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Filter {
    /// The ID of the Filter in the database.
    pub id: FilterId,
    /// A title given by the user to name the filter.
    pub title: String,
    /// The contexts in which the filter should be applied.
    pub context: Vec<FilterContext>,
    /// When the filter should no longer be applied.
    #[serde(with = "iso8601::option")]
    pub expires_at: Option<OffsetDateTime>,
    /// The action to be taken when a status matches this filter.
    pub filter_action: Action,
    /// The keywords grouped under this filter.
    pub keywords: Vec<Keyword>,
    /// The statuses grouped under this filter.
    pub statuses: Vec<Status>,
}

/// Wrapper type for a filter ID string
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct FilterId(String);

impl AsRef<str> for FilterId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Represents the various types of Filter contexts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilterContext {
    /// Represents the "home" context
    Home,
    /// Represents the "notifications" context
    Notifications,
    /// Represents the "public" context
    Public,
    /// Represents the "thread" context
    Thread,
    /// Represents the "account" context
    Account,
}

/// The action the filter should take
///
/// Please note that the spec requests that any unknown value be interpreted
/// as "warn".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    /// Indicates filtered toots should show up, but with a warning
    Warn,
    /// Indicates filtered toots should be hidden.
    Hide,
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FilterActionDeserializer;

        impl<'v> Visitor<'v> for FilterActionDeserializer {
            type Value = Action;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(r#""warn" or "hide" (or really any string; any string other than "hide" will deserialize to "warn")"#)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(if v == "hide" {
                    Action::Hide
                } else {
                    Action::Warn
                })
            }
        }

        deserializer.deserialize_str(FilterActionDeserializer)
    }
}

/// Represents a keyword that, if matched, should cause the filter action to be taken.
///
/// ## Example
/// ```json
/// {
///     "id": "1197",
///     "keyword": "bad word",
///     "whole_word": false
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Keyword {
    /// The ID of the FilterKeyword in the database.
    id: String,
    /// The phrase to be matched against.
    keyword: String,
    /// Should the filter consider word boundaries? See [implementation guidelines
    /// for filters](https://docs.joinmastodon.org/api/guidelines/#filters).
    whole_word: bool,
}

/// Represents a status ID that, if matched, should cause the filter action to be taken.
///
/// ## Example
/// ```json
/// {
///     "id": "1",
///     "status_id": "109031743575371913"
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Status {
    /// The ID of the FilterStatus in the database.
    id: String,
    /// The ID of the filtered Status in the database.
    status_id: String,
}

mod v1 {
    pub use super::FilterContext;
    use serde::{Deserialize, Serialize};
    use time::{serde::iso8601, OffsetDateTime};

    /// Represents a single v1 Filter
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

#[cfg(test)]
mod tests {
    #[cfg(feature = "json")]
    use super::*;

    #[cfg(feature = "json")]
    #[test]
    fn test_filter_action_serialize_and_deserialize() {
        use Action::*;
        let hide = r#""hide""#;
        let warn = r#""warn""#;
        let subject = serde_json::to_string(&Hide).expect("serialize hide");
        assert_eq!(subject, hide);
        let subject = serde_json::to_string(&Warn).expect("serialize warn");
        assert_eq!(subject, warn);
        let subject: Action = serde_json::from_str(hide).expect("deserialize hide");
        assert_eq!(subject, Hide);
        let subject: Action = serde_json::from_str(warn).expect("deserialize warn");
        assert_eq!(subject, Warn);
        let subject: Action =
            serde_json::from_str(r#""something else""#).expect("deserialize something else");
        assert_eq!(subject, Warn);
        // This 👆 further implies...
        let subject: Action = serde_json::from_str(r#""Hide""#).expect("deserialize Hide");
        assert_eq!(subject, Warn /* 👈 !             capital H */);
        // This behavior is specified by the spec https://docs.joinmastodon.org/entities/Filter/#filter_action

        // improper value
        let subject: Result<Action, _> = serde_json::from_str("[1, 2, 3]");
        let subject = subject.expect_err("value was not expected to be valid");
        assert_eq!(
            subject.to_string(),
            r#"invalid type: sequence, expected "warn" or "hide" (or really any string; any string other than "hide" will deserialize to "warn") at line 1 column 0"#
        );
    }
}
