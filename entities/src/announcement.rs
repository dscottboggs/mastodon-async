use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};
use url::Url;

use crate::{custom_emoji::CustomEmoji, status, AccountId, AnnouncementId, StatusId};

/// Represents an announcement set by an administrator.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Announcement/)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Announcement {
    /// The ID of the announcement in the database.
    pub id: AnnouncementId,
    /// The text of the announcement, as HTML.
    pub content: String,
    /// When the announcement will start.
    #[serde(with = "iso8601::option")]
    pub starts_at: Option<OffsetDateTime>,
    /// When the announcement will end.
    #[serde(with = "iso8601::option")]
    pub ends_at: Option<OffsetDateTime>,
    /// Whether the announcement should start and end on dates only instead of
    /// datetimes. Will be false if there is no starts_at or ends_at time.
    pub all_day: bool,
    /// When the announcement was published.
    #[serde(with = "iso8601")]
    pub published_at: OffsetDateTime,
    /// When the announcement was last updated.
    #[serde(with = "iso8601")]
    pub updated_at: OffsetDateTime,
    /// Whether the announcement has been read by the current user.
    #[serde(default)]
    pub read: bool,
    /// Accounts mentioned in the announcement text.
    pub mentions: Vec<Account>,
    /// Statuses linked in the announcement text.
    pub statuses: Vec<Status>,
    /// Tags linked in the announcement text.
    pub tags: Vec<status::Tag>,
    /// Custom emoji used in the announcement text.
    pub emojis: Vec<CustomEmoji>,
    /// Emoji reactions attached to the announcement.
    pub reactions: Vec<Reaction>,
}

/// Represents an emoji reaction to an Announcement.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Reaction/)
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reaction {
    /// The emoji used for the reaction. Either a unicode emoji, or a custom emoji’s shortcode.
    pub name: String,
    /// The total number of users who have added this reaction.
    pub count: i64,
    /// If there is a currently authorized user: Have you added this reaction?
    pub me: Option<bool>,
    /// If the reaction is a custom emoji: A link to the custom emoji.
    pub url: Option<Url>,
    /// If the reaction is a custom emoji: A link to a non-animated version of the custom emoji.
    pub static_url: Option<Url>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    /// The account ID of the mentioned user.
    pub id: AccountId,
    /// The username of the mentioned user.
    pub username: String,
    /// The location of the mentioned user’s profile.
    pub url: Url,
    /// The webfinger acct: URI of the mentioned user. Equivalent to username for local users, or username@domain for remote users.
    pub acct: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Status {
    /// The ID of an attached Status in the database
    pub id: StatusId,
    /// The URL of an attached Status.
    pub url: Url,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let sample = r##"
        {
  "id": "8",
  "content": "<p>Looks like there was an issue processing audio attachments without embedded art since yesterday due to an experimental new feature. That issue has now been fixed, so you may see older posts with audio from other servers pop up in your feeds now as they are being finally properly processed. Sorry!</p>",
  "starts_at": null,
  "ends_at": null,
  "all_day": false,
  "published_at": "2020-07-03T01:27:38.726Z",
  "updated_at": "2020-07-03T01:27:38.752Z",
  "read": true,
  "mentions": [],
  "statuses": [],
  "tags": [],
  "emojis": [],
  "reactions": [
    {
      "name": "bongoCat",
      "count": 9,
      "me": false,
      "url": "https://files.mastodon.social/custom_emojis/images/000/067/715/original/fdba57dff7576d53.png",
      "static_url": "https://files.mastodon.social/custom_emojis/images/000/067/715/static/fdba57dff7576d53.png"
    },
    {
      "name": "thonking",
      "count": 1,
      "me": false,
      "url": "https://files.mastodon.social/custom_emojis/images/000/098/690/original/a8d36edc4a7032e8.png",
      "static_url": "https://files.mastodon.social/custom_emojis/images/000/098/690/static/a8d36edc4a7032e8.png"
    },
    {
      "name": "AAAAAA",
      "count": 1,
      "me": false,
      "url": "https://files.mastodon.social/custom_emojis/images/000/071/387/original/AAAAAA.png",
      "static_url": "https://files.mastodon.social/custom_emojis/images/000/071/387/static/AAAAAA.png"
    },
    {
      "name": "🤔",
      "count": 1,
      "me": true
    }
  ]
}"##;
        let ann: Announcement = serde_json::from_str(sample).expect("deserialize");
        assert_eq!(ann.id, AnnouncementId::new("8"));
        assert_eq!(ann.content, "<p>Looks like there was an issue processing audio attachments without embedded art since yesterday due to an experimental new feature. That issue has now been fixed, so you may see older posts with audio from other servers pop up in your feeds now as they are being finally properly processed. Sorry!</p>");
        assert!(ann.starts_at.is_none());
        assert!(ann.ends_at.is_none());
        assert!(!ann.all_day);
        let reaction = &ann.reactions[0];
        assert_eq!(reaction.name, "bongoCat");
        assert_eq!(reaction.count, 9);
        assert!(reaction.me.is_some());
        assert!(!reaction.me.unwrap());
        assert_eq!(reaction.url.as_ref().map(|it| it.as_ref()), Some("https://files.mastodon.social/custom_emojis/images/000/067/715/original/fdba57dff7576d53.png"));
    }

    #[test]
    fn test_reaction_standard_emoji() {
        let example = r#"{
          "name": "🤔",
          "count": 1,
          "me": true
        }"#;
        let subject: Reaction = serde_json::from_str(example).expect("deserialize");
        assert_eq!(subject.name, "🤔");
        assert_eq!(subject.count, 1);
        assert!(subject.me.expect("me"));
    }
    #[test]
    fn test_reaction_custom_emoji() {
        let example = r#"{
          "name": "bongoCat",
          "count": 9,
          "me": false,
          "url": "https://files.mastodon.social/custom_emojis/images/000/067/715/original/fdba57dff7576d53.png",
          "static_url": "https://files.mastodon.social/custom_emojis/images/000/067/715/static/fdba57dff7576d53.png"
        }"#;
        let subject: Reaction = serde_json::from_str(example).expect("deserialize");
        assert_eq!(subject.name, "bongoCat");
        assert_eq!(subject.count, 9);
        assert!(!subject.me.expect("me"));
        assert_eq!(subject.url.expect("url").as_ref(), "https://files.mastodon.social/custom_emojis/images/000/067/715/original/fdba57dff7576d53.png");
        assert_eq!(subject.static_url.expect("static url").as_ref(), "https://files.mastodon.social/custom_emojis/images/000/067/715/static/fdba57dff7576d53.png");
    }
}
