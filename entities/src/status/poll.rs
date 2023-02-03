use derive_builder::Builder;
use serde::{Serialize, Deserialize};
use time::{serde::iso8601, OffsetDateTime};

use crate::prelude::CustomEmoji;

/// Represents a poll attached to a status.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Poll/)
#[derive(Debug, Builder, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[builder(build_fn(error = "crate::error::Error"))]
pub struct Poll {
    /// The ID of the poll in the database.
    pub id: String,
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
    pub own_votes: Vec<u16>,
}

#[derive(Debug, Builder, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PollOption {
    /// The text value of the poll option.
    pub title: String,
    /// The total number of received votes for this option. `None` if the
    /// results aren't published yet.
    pub votes_count: Option<u64>,
}
