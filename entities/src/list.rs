use derive_is_enum_variant::is_enum_variant;
use serde::{Deserialize, Serialize};

use crate::ListId;

/// Represents a list of some users that the authenticated user follows.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/List/)
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct List {
    /// The internal database ID of the list.
    pub id: ListId,
    /// The user-defined title of the list.
    pub title: String,
    /// Which replies should be shown in the list.
    pub replies_policy: RepliesPolicy,
}

/// Which replies should be shown in the list.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, is_enum_variant)]
#[serde(rename_all = "lowercase")]
pub enum RepliesPolicy {
    /// Show replies to any followed user
    Followed,
    /// Show replies to members of the list
    List,
    /// Show replies to no one
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let example = r#"{
          "id": "13585",
          "title": "test",
          "replies_policy": "list"
        }"#;
        let subject: List = serde_json::from_str(example).expect("deserialize");
        assert_eq!(subject.id, ListId::new("13585"));
        assert_eq!(subject.title, "test");
        assert!(subject.replies_policy.is_list());
    }
}
