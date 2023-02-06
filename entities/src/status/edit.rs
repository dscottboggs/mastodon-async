use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

use crate::{
    account::Account,
    prelude::{Attachment, CustomEmoji},
};

/// Represents a revision of a status that has been edited.
/// 
/// See also [the API documentation](https://docs.joinmastodon.org/entities/StatusEdit/)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Edit {
    /// The content of the status at this revision.
    pub content: String,
    /// The content of the subject or content warning at this revision.
    pub spoiler_text: String,
    /// Whether the status was marked sensitive at this revision.
    pub sensitive: bool,
    /// The timestamp of when the revision was published.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// The account that published this revision.
    pub account: Account,
    /// The current state of the poll options at this revision. Note that edits
    /// changing the poll options will be collapsed together into one edit,
    /// since this action resets the poll.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<PollEdit>,
    /// Media currently attached to this status
    pub media_attachments: Vec<Attachment>,
    /// Any custom emoji that are used in the current revision.
    pub emojis: Vec<CustomEmoji>,
}

/// The current state of the poll options at this revision. Note that edits
/// changing the poll options will be collapsed together into one edit,
/// since this action resets the poll.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PollEdit {
    /// The poll options at this revision
    pub options: Vec<PollEditOption>,
}

/// The poll options at this revision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PollEditOption {
    /// The text for a poll option.
    pub title: String,
}

#[cfg(test)]
mod tests {
    use time::{format_description::well_known::Iso8601, OffsetDateTime};

    use super::*;

    #[test]
    fn test_deserialize() {
        let example = r#"{
            "content": "<p>this is a status that has been edited three times. this time a poll has been added.</p>",
            "spoiler_text": "",
            "sensitive": false,
            "created_at": "2022-09-05T00:03:32.480Z",
            "poll": {
                "options": [
                    {
                        "title": "cool"
                    },
                    {
                        "title": "uncool"
                    },
                    {
                        "title": "spiderman (this option has been changed)"
                    }
                ]
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
            "emojis": []
        }"#;
        let subject: Edit = serde_json::from_str(example).expect("deserialize");
        assert_eq!(subject.content, "<p>this is a status that has been edited three times. this time a poll has been added.</p>");
        assert!(subject.spoiler_text.is_empty());
        assert!(!subject.sensitive);
        assert_eq!(
            subject.created_at,
            OffsetDateTime::parse("2022-09-05T00:03:32.480Z", &Iso8601::PARSING)
                .expect("parse create time")
        );
        let poll_options = subject.poll.expect("poll").options;
        for (i, title) in ["cool", "uncool", "spiderman (this option has been changed)"]
            .iter()
            .enumerate()
        {
            assert_eq!(poll_options[i].title, *title);
        }
        assert_eq!(poll_options.len(), 3);
        assert!(subject.media_attachments.is_empty());
        assert!(subject.emojis.is_empty());
    }
}
