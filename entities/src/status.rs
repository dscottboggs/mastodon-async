//! Module containing all info relating to a status.

use crate::custom_emoji::CustomEmoji;

use super::prelude::*;
use derive_builder::Builder;
use isolang::Language;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// A status from the instance.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Status {
    /// The ID of the status.
    pub id: StatusId,
    /// A Fediverse-unique resource ID.
    pub uri: String,
    /// URL to the status page (can be remote)
    pub url: Option<String>,
    /// The Account which posted the status.
    pub account: Account,
    /// The ID of the status this status is replying to, if the status is
    /// a reply.
    pub in_reply_to_id: Option<String>,
    /// The ID of the account this status is replying to, if the status is
    /// a reply.
    pub in_reply_to_account_id: Option<String>,
    /// If this status is a reblogged Status of another User.
    pub reblog: Option<Box<Status>>,
    /// Body of the status; this will contain HTML
    /// (remote HTML already sanitized)
    pub content: String,
    /// The time the status was created.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// An array of Emoji
    pub emojis: Vec<Emoji>,
    /// The numbef or replies to this status.
    pub replies_count: Option<u64>,
    /// The number of reblogs for the status.
    pub reblogs_count: u64,
    /// The number of favourites for the status.
    pub favourites_count: u64,
    /// Whether the application client has reblogged the status.
    pub reblogged: Option<bool>,
    /// Whether the application client has favourited the status.
    pub favourited: Option<bool>,
    /// Whether media attachments should be hidden by default.
    pub sensitive: bool,
    /// If not empty, warning text that should be displayed before the actual
    /// content.
    pub spoiler_text: String,
    /// The visibilty of the status.
    pub visibility: Visibility,
    /// An array of attachments.
    pub media_attachments: Vec<Attachment>,
    /// An array of mentions.
    pub mentions: Vec<Mention>,
    /// An array of tags.
    pub tags: Vec<Tag>,
    /// The associated card
    pub card: Option<Card>,
    /// Name of application used to post status.
    pub application: Option<Application>,
    /// The detected language for the status, if detected.
    pub language: Option<String>,
    /// Whether this is the pinned status for the account that posted it.
    pub pinned: Option<bool>,
}

/// A mention of another user.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Mention {
    /// URL of user's profile (can be remote).
    pub url: String,
    /// The username of the account.
    pub username: String,
    /// Equals `username` for local users, includes `@domain` for remote ones.
    pub acct: String,
    /// Account ID.
    pub id: String,
}

/// Struct representing an emoji within text.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Emoji {
    /// The shortcode of the emoji
    pub shortcode: String,
    /// URL to the emoji static image
    pub static_url: String,
    /// URL to the emoji image
    pub url: String,
}

/// Hashtags in the status. This functions both as a
/// [`Status::Tag`](https://docs.joinmastodon.org/entities/Status/#Tag), and
/// as a [`Tag`](https://docs.joinmastodon.org/entities/Tag/). In the case of
/// the former, at the time of writing, the history field is always empty and
/// the following field is always none.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    /// The hashtag, not including the preceding `#`.
    pub name: String,
    /// The URL of the hashtag.
    pub url: String,
    /// Usage statistics for given days (typically the past week).
    #[serde(default = "Vec::new")]
    pub history: Vec<TagHistory>,
    /// Whether the current token’s authorized user is following this tag.
    pub following: Option<bool>,
}

/// Application details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Application {
    /// Name of the application.
    pub name: String,
    /// Homepage URL of the application.
    pub website: Option<String>,
}

/// Usage statistics for given days (typically the past week).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TagHistory {
    /// UNIX timestamp on midnight of the given day.
    pub day: String,
    /// The counted usage of the tag within that day.
    pub uses: String,
    /// The total of accounts using the tag within that day.
    pub accounts: String,
}

/// Represents a poll attached to a status.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Poll/)
#[derive(Debug, Builder, Default, Clone, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Builder, Default, Clone, Serialize, PartialEq, Eq)]
pub struct PollOption {
    /// The text value of the poll option.
    pub title: String,
    /// The total number of received votes for this option. `None` if the
    /// results aren't published yet.
    pub votes_count: Option<u64>,
}

/// Represents a post that can be sent to the POST /api/v1/status endpoint
///
/// See also [the API documentation](https://docs.joinmastodon.org/methods/statuses/#form-data-parameters)
#[derive(Debug, Builder, Default, Clone, Serialize, PartialEq, Eq)]
#[builder(build_fn(error = "crate::error::Error"))]
pub struct NewStatus {
    /// The text content of the status. If media_ids is provided, this becomes
    /// optional. Attaching a poll is optional while status is provided.
    ///
    /// Note that this means there is at this time no check provided by this
    /// type to ensure that this value is set when it is required by the API,
    /// and an APIError should be expected from [`crate::Mastodon::new_status()`]
    /// in this case.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub status: Option<String>,
    /// ID of the status being replied to, if status is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub in_reply_to_id: Option<String>,
    /// Include Attachment IDs to be attached as media. If provided, status
    /// becomes optional, and poll cannot be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub media_ids: Option<Vec<AttachmentId>>,
    /// Mark status and attached media as sensitive? Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sensitive: Option<bool>,
    /// Text to be shown as a warning or subject before the actual content.
    /// Statuses are generally collapsed behind this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub spoiler_text: Option<String>,
    /// Sets the visibility of the posted status to public, unlisted, private, direct.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub visibility: Option<Visibility>,
    /// ISO 639 language code for this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub language: Option<Language>,
    /// Ignored by Mastodon servers, sets the content type for the status.
    /// Mastodon "toots" are always `text/plain`, regardless of this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub content_type: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use isolang::Language;
    use serde_json;

    #[test]
    fn test_new() {
        let s = NewStatusBuilder::default()
            .status("a status")
            .build()
            .expect("Couldn't build status");
        let expected = NewStatus {
            status: Some("a status".to_string()),
            in_reply_to_id: None,
            media_ids: None,
            sensitive: None,
            spoiler_text: None,
            visibility: None,
            language: None,
            content_type: None,
        };
        assert_eq!(s, expected);
    }

    #[test]
    fn test_default_visibility() {
        let v: Visibility = Default::default();
        assert_eq!(v, Visibility::Public);
    }

    #[test]
    fn test_serialize_visibility() {
        assert_eq!(
            serde_json::to_string(&Visibility::Direct).expect("couldn't serialize visibility"),
            "\"direct\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&Visibility::Private).expect("couldn't serialize visibility"),
            "\"private\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&Visibility::Unlisted).expect("couldn't serialize visibility"),
            "\"unlisted\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&Visibility::Public).expect("couldn't serialize visibility"),
            "\"public\"".to_string()
        );
    }

    #[test]
    fn test_serialize_status() {
        let status = NewStatusBuilder::default()
            .status("a status")
            .build()
            .expect("Couldn't build status");
        assert_eq!(
            serde_json::to_string(&status).expect("Couldn't serialize status"),
            "{\"status\":\"a status\"}".to_string()
        );

        let status = NewStatusBuilder::default()
            .status("a status")
            .language(Language::Eng)
            .build()
            .expect("Couldn't build status");
        assert_eq!(
            serde_json::to_string(&status).expect("Couldn't serialize status"),
            "{\"status\":\"a status\",\"language\":\"eng\"}"
        );
    }
}
