//! Module containing all info relating to a status.

pub mod edit;
/// For building a new status
pub mod new;
pub mod poll;
pub mod scheduled;

pub use edit::Edit;
use isolang::Language;
pub use new::{NewStatus, NewStatusBuilder};
pub use poll::{Poll, PollBuilder};
pub use scheduled::Status as Scheduled;

use crate::{custom_emoji::CustomEmoji, filter};

use super::prelude::*;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};
use url::Url;

/// Represents a status posted by an account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Status {
    /// The ID of the status.
    pub id: StatusId,
    /// A Fediverse-unique resource ID.
    pub uri: Url,
    /// A link to the status’s HTML representation.
    pub url: Option<Url>,
    /// The Account which posted the status.
    pub account: Account,
    /// The ID of the status this status is replying to, if the status is
    /// a reply.
    pub in_reply_to_id: Option<StatusId>,
    /// The ID of the account this status is replying to, if the status is
    /// a reply.
    pub in_reply_to_account_id: Option<AccountId>,
    /// If this status is a reblogged Status of another User.
    pub reblog: Option<Box<Status>>,
    /// Body of the status; this will contain HTML
    /// (remote HTML already sanitized)
    pub content: String,
    /// The time the status was created.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// Timestamp of when the status was last edited.
    #[serde(
        with = "iso8601::option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub edited_at: Option<OffsetDateTime>,
    /// Custom emoji to be used when rendering status content.
    pub emojis: Vec<CustomEmoji>,
    /// The number of replies to this status.
    pub replies_count: u64,
    /// How many boosts this status has received.
    pub reblogs_count: u64,
    /// The number of favourites for the status.
    pub favourites_count: u64,
    /// Whether the application client has reblogged the status.
    pub reblogged: Option<bool>,
    /// Whether the application client has favourited the status.
    pub favourited: Option<bool>,
    /// If the current token has an authorized user: Have you muted
    /// notifications for this status’s conversation?
    pub muted: Option<bool>,
    /// If the current token has an authorized user: Have you bookmarked this
    /// status?
    pub bookmarked: Option<bool>,
    /// If the current token has an authorized user: Have you pinned this
    /// status? Only appears if the status is pinnable.
    pub pinned: Option<bool>,
    /// Whether media attachments should be hidden by default.
    pub sensitive: bool,
    /// If not empty, warning text that should be displayed before the actual
    /// content.
    pub spoiler_text: String,
    /// The visibilty of the status.
    pub visibility: Visibility,
    /// An array of attachments.
    pub media_attachments: Vec<Attachment>,
    /// Hashtags used within the status content.
    pub mentions: Vec<Mention>,
    /// An array of tags.
    pub tags: Vec<Tag>,
    /// Media that is attached to this status.
    pub application: Option<Application>,
    /// The detected language for the status, if detected.
    pub language: Option<Language>,
    /// The poll attached to the status.
    pub poll: Option<Poll>,
    /// Preview card for links included within status content.
    pub card: Option<Card>,
    /// Plain-text source of a status. Returned instead of content when status
    /// is deleted, so the user may redraft from the source text without the
    /// client having to reverse-engineer the original text from the HTML
    /// content.
    pub text: Option<String>,
    /// If the current token has an authorized user: The filter and keywords
    /// that matched this status.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filtered: Vec<filter::Result>,
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

#[cfg(test)]
mod tests {
    use time::format_description::well_known::Iso8601;

    use super::*;

    #[test]
    fn test_deserialize_example() {
        let example = r#"{
            "id": "103270115826048975",
            "created_at": "2019-12-08T03:48:33.901Z",
            "in_reply_to_id": null,
            "in_reply_to_account_id": null,
            "sensitive": false,
            "spoiler_text": "",
            "visibility": "public",
            "language": "en",
            "uri": "https://mastodon.social/users/Gargron/statuses/103270115826048975",
            "url": "https://mastodon.social/@Gargron/103270115826048975",
            "replies_count": 5,
            "reblogs_count": 6,
            "favourites_count": 11,
            "favourited": false,
            "reblogged": false,
            "muted": false,
            "bookmarked": false,
            "content": "<p>&quot;I lost my inheritance with one wrong digit on my sort code&quot;</p><p><a href=\"https://www.theguardian.com/money/2019/dec/07/i-lost-my-193000-inheritance-with-one-wrong-digit-on-my-sort-code\" rel=\"nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://www.</span><span class=\"ellipsis\">theguardian.com/money/2019/dec</span><span class=\"invisible\">/07/i-lost-my-193000-inheritance-with-one-wrong-digit-on-my-sort-code</span}</p>",
            "reblog": null,
            "application": {
              "name": "Web",
              "website": null
            },
            "account": {
              "id": "1",
              "username": "Gargron",
              "acct": "Gargron",
              "display_name": "Eugen",
              "locked": false,
              "bot": false,
              "discoverable": true,
              "group": false,
              "created_at": "+002016-03-16T14:34:26.392000000Z",
              "note": "<p>Developer of Mastodon and administrator of mastodon.social. I post service announcements, development updates, and personal stuff.</p>",
              "url": "https://mastodon.social/@Gargron",
              "avatar": "https://files.mastodon.social/accounts/avatars/000/000/001/original/d96d39a0abb45b92.jpg",
              "avatar_static": "https://files.mastodon.social/accounts/avatars/000/000/001/original/d96d39a0abb45b92.jpg",
              "header": "https://files.mastodon.social/accounts/headers/000/000/001/original/c91b871f294ea63e.png",
              "header_static": "https://files.mastodon.social/accounts/headers/000/000/001/original/c91b871f294ea63e.png",
              "followers_count": 322930,
              "following_count": 459,
              "statuses_count": 61323,
              "last_status_at": "2019-12-10T08:14:44.811Z",
              "emojis": [],
              "fields": [
                {
                  "name": "Patreon",
                  "value": "<a href=\"https://www.patreon.com/mastodon\" rel=\"me nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://www.</span><span class=\"\">patreon.com/mastodon</span><span class=\"invisible\"></span}",
                  "verified_at": null
                },
                {
                  "name": "Homepage",
                  "value": "<a href=\"https://zeonfederated.com\" rel=\"me nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://</span><span class=\"\">zeonfederated.com</span><span class=\"invisible\"></span}",
                  "verified_at": "+002019-07-15T18:29:57.191000000Z"
                }
              ]
            },
            "media_attachments": [],
            "mentions": [],
            "tags": [],
            "emojis": [],
            "card": {
              "url": "https://www.theguardian.com/money/2019/dec/07/i-lost-my-193000-inheritance-with-one-wrong-digit-on-my-sort-code",
              "title": "‘I lost my £193,000 inheritance – with one wrong digit on my sort code’",
              "description": "When Peter Teich’s money went to another Barclays customer, the bank offered £25 as a token gesture",
              "type": "link",
              "author_name": "",
              "author_url": "",
              "provider_name": "",
              "provider_url": "",
              "html": "",
              "width": 0,
              "height": 0,
              "image": null,
              "embed_url": ""
            },
            "poll": null
        }"#;
        let status: Status = serde_json::from_str(example).expect("deserialize");
        assert_eq!(status.id, StatusId::new("103270115826048975"));
        assert_eq!(
            status.created_at,
            OffsetDateTime::parse("2019-12-08T03:48:33.901Z", &Iso8601::PARSING)
                .expect("parse time example")
        );
        assert!(status.in_reply_to_id.is_none());
        assert!(status.in_reply_to_account_id.is_none());
        assert!(!status.sensitive);
        assert!(
            status.spoiler_text.is_empty(),
            "spoiler text was {:?}",
            status.spoiler_text
        );
        assert!(status.visibility.is_public());
        assert_eq!(status.language, Some(Language::Eng));
        assert_eq!(
            status.uri.as_ref(),
            "https://mastodon.social/users/Gargron/statuses/103270115826048975"
        );
        assert_eq!(
            status.url.expect("url").as_ref(),
            "https://mastodon.social/@Gargron/103270115826048975"
        );
        assert_eq!(status.replies_count, 5);
        assert_eq!(status.reblogs_count, 6);
        assert_eq!(status.favourites_count, 11);
        assert!(!status.favourited.expect("favourited"));
        assert!(!status.reblogged.expect("reblogged"));
        assert!(!status.muted.expect("muted"));
        assert!(!status.bookmarked.expect("bookmarked"));
        assert_eq!(status.content, "<p>&quot;I lost my inheritance with one wrong digit on my sort code&quot;</p><p><a href=\"https://www.theguardian.com/money/2019/dec/07/i-lost-my-193000-inheritance-with-one-wrong-digit-on-my-sort-code\" rel=\"nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://www.</span><span class=\"ellipsis\">theguardian.com/money/2019/dec</span><span class=\"invisible\">/07/i-lost-my-193000-inheritance-with-one-wrong-digit-on-my-sort-code</span}</p>");
        assert!(status.reblog.is_none());
        let app = status.application.expect("application");
        assert_eq!(app.name, "Web");
        assert!(
            app.website.is_none(),
            "subject.application.website was {:?}",
            app.website
        );
        let acct = status.account;
        assert_eq!(acct.id, AccountId::new("1"));
        assert_eq!(acct.username, "Gargron");
        assert_eq!(acct.acct, "Gargron");
        assert_eq!(acct.display_name, "Eugen");
        assert!(!acct.locked);
        assert!(!acct.bot.expect("bot"));
        assert!(acct.discoverable.expect("discoverable"));
        assert!(!acct.group);
        assert_eq!(
            acct.created_at,
            OffsetDateTime::parse("2016-03-16T14:34:26.392Z", &Iso8601::PARSING)
                .expect("acct creation time parse")
        );
        assert_eq!(acct.note, "<p>Developer of Mastodon and administrator of mastodon.social. I post service announcements, development updates, and personal stuff.</p>");
        assert_eq!(acct.url.as_ref(), "https://mastodon.social/@Gargron");
        assert_eq!(acct.avatar.as_ref(), "https://files.mastodon.social/accounts/avatars/000/000/001/original/d96d39a0abb45b92.jpg");
        assert_eq!(acct.avatar, acct.avatar_static);
        assert_eq!(acct.header.as_ref(), "https://files.mastodon.social/accounts/headers/000/000/001/original/c91b871f294ea63e.png");
        assert_eq!(acct.header, acct.header_static);
        assert_eq!(acct.followers_count, 322930);
        assert_eq!(acct.following_count, 459);
        assert_eq!(acct.statuses_count, 61323);
        assert_eq!(acct.last_status_at, Some("2019-12-10T08:14:44.811Z".into()));
        assert!(acct.emojis.is_empty());
        let field = acct.fields.get(0).expect("first field");
        assert_eq!(field.name, "Patreon");
        assert_eq!(field.value, "<a href=\"https://www.patreon.com/mastodon\" rel=\"me nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://www.</span><span class=\"\">patreon.com/mastodon</span><span class=\"invisible\"></span}");
        assert!(field.verified_at.is_none());
        let field = acct.fields.get(1).expect("second field");
        assert_eq!(field.name, "Homepage");
        assert_eq!(field.value, "<a href=\"https://zeonfederated.com\" rel=\"me nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://</span><span class=\"\">zeonfederated.com</span><span class=\"invisible\"></span}");
        assert_eq!(
            field.verified_at,
            Some(
                OffsetDateTime::parse("2019-07-15T18:29:57.191+00:00", &Iso8601::PARSING)
                    .expect("parse field verified time")
            )
        );
        assert!(status.media_attachments.is_empty());
        assert!(status.mentions.is_empty());
        assert!(status.tags.is_empty());
        assert!(status.emojis.is_empty());
        let card = status.card.expect("card");
        assert_eq!(card.url.as_str(), "https://www.theguardian.com/money/2019/dec/07/i-lost-my-193000-inheritance-with-one-wrong-digit-on-my-sort-code");
        assert_eq!(
            card.title,
            "‘I lost my £193,000 inheritance – with one wrong digit on my sort code’"
        );
        assert_eq!(card.description, "When Peter Teich’s money went to another Barclays customer, the bank offered £25 as a token gesture");
        assert!(card.card_type.is_link());
        assert!(card.author_name.is_empty());
        assert!(card.author_url.is_none());
        assert!(card.provider_name.is_empty());
        assert!(card.provider_url.is_none());
        assert!(card.html.is_empty());
        assert_eq!(card.width, 0);
        assert_eq!(card.height, 0);
        assert!(card.image.is_none());
        assert!(card.embed_url.is_none());
        assert!(status.poll.is_none());
    }
}
