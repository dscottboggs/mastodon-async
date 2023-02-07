//! module containing everything relating to a relationship with
//! another account.
use isolang::Language;
use serde::{Deserialize, Serialize};

use crate::RelationshipId;

/// Represents the relationship between accounts, such as following / blocking / muting / etc.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Relationship/)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Relationship {
    /// Target account id
    pub id: RelationshipId,
    /// Are you following this user?
    pub following: bool,
    /// Are you followed by this user?
    pub followed_by: bool,
    /// Are you blocking this user?
    pub blocking: bool,
    /// Is this user blocking you?
    pub blocked_by: bool,
    /// Are you muting this user?
    pub muting: bool,
    /// Do you have a pending follow request for this user?
    pub requested: bool,
    /// Are you muting notifications from this user?
    pub muting_notifications: bool,
    /// Are you blocking this user’s domain?
    pub domain_blocking: bool,
    /// Are you receiving this user’s boosts in your home timeline?
    pub showing_reblogs: bool,
    /// Have you enabled notifications for this user?
    pub notifying: bool,
    /// Which languages are you following from this user?
    #[serde(default)]
    pub languages: Vec<Language>,
    /// Are you featuring this user on your profile?
    #[serde(default)]
    pub endorsed: bool,
    /// This user’s profile bio
    pub note: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let example = r#"{
          "id": "1",
          "following": true,
          "showing_reblogs": true,
          "notifying": false,
          "followed_by": true,
          "blocking": false,
          "blocked_by": false,
          "muting": false,
          "muting_notifications": false,
          "requested": false,
          "domain_blocking": false,
          "endorsed": false,
          "note": ""
        }"#;
        let subject: Relationship = serde_json::from_str(example).expect("deserialize");
        assert_eq!(subject.id, RelationshipId::new("1"));
        assert!(subject.following);
        assert!(subject.showing_reblogs);
        assert!(!subject.notifying);
        assert!(subject.followed_by);
        assert!(!subject.blocking);
        assert!(!subject.blocked_by);
        assert!(!subject.muting);
        assert!(!subject.muting_notifications);
        assert!(!subject.requested);
        assert!(!subject.domain_blocking);
        assert!(!subject.endorsed);
        assert!(subject.note.is_empty());
    }
}
