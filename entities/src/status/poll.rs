use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

use crate::{prelude::CustomEmoji, PollId};

/// Represents a poll attached to a status.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Poll/)
#[derive(Debug, Builder, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[builder(build_fn(error = "crate::error::Error"))]
pub struct Poll {
    /// The ID of the poll in the database.
    pub id: PollId,
    /// When the poll ends.
    #[serde(with = "iso8601::option")]
    pub expires_at: Option<OffsetDateTime>,
    /// Is the poll currently expired?
    pub expired: bool,
    /// Does the poll allow multiple-choice answers?
    pub multiple: bool,
    /// How many votes have been received.
    pub votes_count: u64,
    /// How many unique accounts have voted on a multiple-choice poll. `None`
    /// if [`multiple`] is `false`.
    pub voters_count: Option<u64>,
    /// Possible answers for the poll.
    pub options: Vec<PollOption>,
    /// Custom emoji to be used for rendering poll options.
    pub emojis: Vec<CustomEmoji>,
    /// When called with a user token, has the authorized user voted?
    pub voted: Option<bool>,
    /// When called with a user token, which options has the authorized user
    /// chosen? Contains an array of index values for options.
    #[serde(default)]
    pub own_votes: Vec<usize>,
}

#[derive(Debug, Builder, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PollOption {
    /// The text value of the poll option.
    pub title: String,
    /// The total number of received votes for this option. `None` if the
    /// results aren't published yet.
    pub votes_count: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let example = r#"{
          "id": "34830",
          "expires_at": "2019-12-05T04:05:08.302Z",
          "expired": true,
          "multiple": false,
          "votes_count": 10,
          "voters_count": null,
          "voted": true,
          "own_votes": [
            1
          ],
          "options": [
            {
              "title": "accept",
              "votes_count": 6
            },
            {
              "title": "deny",
              "votes_count": 4
            }
          ],
          "emojis": []
        }"#;
        let poll: Poll = serde_json::from_str(example).unwrap();
        assert_eq!(poll.id, PollId::new("34830"));
        assert!(poll.expired);
        assert!(!poll.multiple);
        assert_eq!(poll.votes_count, 10);
        assert!(poll.voters_count.is_none());
        assert!(poll.voted.is_some());
        assert!(poll.voted.unwrap());
        assert_eq!(poll.own_votes, [1]);
        let option = &poll.options[0];
        assert_eq!(option.title, "accept");
        assert_eq!(option.votes_count, Some(6));
        let option = &poll.options[1];
        assert_eq!(option.title, "deny");
        assert_eq!(option.votes_count, Some(4));
        assert!(poll.emojis.is_empty());
    }
}
