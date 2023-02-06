//! Module containing everything related to an instance.
use isolang::Language;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};
use url::Url;

use crate::{account::Account, admin, conversion, RuleId};

/// Represents the software instance of Mastodon running on this domain.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Instance/)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instance {
    /// The domain name of the instance.
    pub domain: String,
    /// The title of the website.
    pub title: String,
    /// The version of Mastodon installed on the instance.
    pub version: String,
    /// The URL for the source code of the software running on this instance, in keeping with AGPL license requirements.
    pub source_url: String,
    /// A short, plain-text description defined by the admin.
    pub description: String,
    /// Usage data for this instance.
    pub usage: Usage,
    /// An image used to represent this instance.
    pub thumbnail: Thumbnail,
    /// Primary languages of the website and its staff.
    pub languages: Vec<Language>,
    /// Configured values and limits for this website.
    pub configuration: Configuration,
    /// Information about registering for this website.
    pub registrations: Registrations,
    /// Hints related to contacting a representative of the website.
    pub contact: Contact,
    /// An itemized list of rules for this website.
    pub rules: Vec<Rule>,
}

/// Usage data for this instance.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Usage {
    /// Usage data related to users on this instance.
    pub users: Users,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Users {
    pub active_month: i64,
}

/// An image used to represent this instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Thumbnail {
    /// The URL for the thumbnail image.
    pub url: String,
    /// A hash computed by [the BlurHash algorithm](https://github.com/woltapp/blurhash),
    /// for generating colorful preview thumbnails when media has not been
    /// downloaded yet.
    pub blurhash: String,
    /// Links to scaled resolution images, for high DPI screens.
    pub versions: ThumbnailVersions,
}

/// Links to scaled resolution images, for high DPI screens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThumbnailVersions {
    /// The URL for the thumbnail image at 1x resolution.
    #[serde(rename = "@1x")]
    pub at_1x: Url,
    /// The URL for the thumbnail image at 2x resolution.
    #[serde(rename = "@2x")]
    pub at_2x: Url,
}

/// Hints related to contacting a representative of the website.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    /// An email address that can be messaged regarding inquiries or issues.
    pub email: String,
    /// An account that can be contacted natively over the network regarding inquiries or issues.
    pub account: Account,
}

/// Information about registering for this website.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Registrations {
    /// Whether registrations are enabled.
    pub enabled: bool,
    /// Whether registrations require moderator approval.
    pub approval_required: bool,
    /// A custom, HTML-formatted message to be shown when registrations are closed.
    pub message: Option<String>,
}
/// Represents a rule that server users should follow.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Rule/)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Rule {
    /// An identifier for the rule.
    pub id: RuleId,
    /// The rule to be followed.
    pub text: String,
}

/// Represents a domain that is blocked by the instance.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/DomainBlock/)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DomainBlock {
    /// The domain which is blocked. This may be obfuscated or partially censored.
    pub domain: String,
    /// The SHA256 hash digest of the domain string.
    pub digest: String,
    /// The level to which the domain is blocked.
    pub severity: admin::domain::BlockSeverity,
    /// Admin's public comment, an optional reason for the domain block.
    pub comment: Option<String>,
}

/// Weekly activity on an instance
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Activity {
    /// UNIX Timestamp at midnight at the first day of the week.
    pub week: String,
    /// The number of Statuses created since the week began (cast from an integer)
    #[serde(with = "conversion::string_to_u64")]
    pub statuses: u64,
    /// The number of user logins since the week began (cast from an integer)
    #[serde(with = "conversion::string_to_u64")]
    pub logins: u64,
    /// The number of user registrations since the week began (cast from an integer)
    #[serde(with = "conversion::string_to_u64")]
    pub registrations: u64,
}

/// Configured values and limits for this website.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Configuration {
    /// URLs of interest for clients apps.
    pub urls: configuration::Urls,
    /// Limits related to accounts.
    pub accounts: v1::configuration::Accounts,
    /// Limits related to authoring statuses.
    pub statuses: v1::configuration::Statuses,
    /// Hints for which attachments will be accepted.
    pub media_attachments: v1::configuration::MediaAttachments,
    /// Hints for which attachments will be accepted.
    pub polls: v1::configuration::Polls,
    /// Hints related to translation.
    pub translation: configuration::Translation,
}

/// Represents an extended description for the instance, to be shown on its about page.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/ExtendedDescription/)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ExtendedDescription {
    /// A timestamp of when the extended description was last updated.
    #[serde(with = "iso8601")]
    pub updated_at: OffsetDateTime,
    /// The rendered HTML content of the extended description.
    pub content: String,
}

pub mod configuration {
    use serde::{Deserialize, Serialize};
    use url::Url;

    /// Url configurations
    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Urls {
        /// Url for streaming API, typically a `wss://` url.
        pub streaming: Url,
    }

    /// Hints related to translation.
    #[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Translation {
        /// Whether the Translations API is available on this instance.
        pub enabled: bool,
    }
}

pub mod v1 {
    use isolang::Language;
    use url::Url;

    use super::*;

    /// Obsolete type, see [`super::Instance`]. Represents the software instance
    /// of Mastodon running on this domain.
    ///
    /// See also [the API documentation](https://docs.joinmastodon.org/entities/V1_Instance/)
    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
    pub struct Instance {
        /// URI of the current instance
        pub uri: String,
        /// The instance's title.
        pub title: String,
        /// A description for the instance.
        pub description: String,
        /// A short, plain-text description defined by the admin.
        pub short_description: String,
        /// An email address which can be used to contact the
        /// instance administrator.
        pub email: String,
        /// The Mastodon version used by instance.
        pub version: String,
        /// Urls to the streaming api.
        pub urls: Option<Urls>,
        /// Stats about the instance.
        pub stats: Option<Stats>,
        /// Thumbnail of the server image.
        pub thumbnail: Option<Url>,
        /// List of languages used on the server.
        pub languages: Option<Vec<Language>>,
        /// Whether registrations are enabled.
        pub registrations: bool,
        /// Whether registrations require moderator approval.
        pub approval_required: bool,
        /// A user that can be contacted, as an alternative to email.
        pub contact_account: Account,
        /// An itemized list of rules for this website.  
        pub rules: Vec<Rule>,
        /// Configured values and limits for this website.
        pub configuration: Configuration,
    }

    /// Statistics about the Mastodon instance.
    #[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Stats {
        /// Total users on this instance.
        pub user_count: u64,
        /// Total statuses on this instance.
        pub status_count: u64,
        /// Total domains discovered by this instance.
        pub domain_count: u64,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Configuration {
        /// Limits related to accounts.
        pub accounts: Option<configuration::Accounts>,
        /// Limits related to authoring statuses.
        pub statuses: configuration::Statuses,
        /// Hints for which attachments will be accepted.
        pub media_attachments: configuration::MediaAttachments,
    }

    /// Url configurations
    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Urls {
        /// Url for streaming API, typically a `wss://` url.
        pub streaming_api: Url,
    }

    pub mod configuration {
        use serde::{Deserialize, Serialize};

        /// Limits related to accounts.
        #[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
        pub struct Accounts {
            /// The maximum number of featured tags allowed for each account.
            pub max_featured_tags: i64,
        }
        /// Limits related to authoring statuses.
        #[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
        pub struct Statuses {
            /// The maximum number of allowed characters per status.
            pub max_characters: i64,
            /// The maximum number of media attachments that can be added to a status.
            pub max_media_attachments: i64,
            /// Each URL in a status will be assumed to be exactly this many characters.
            pub characters_reserved_per_url: i64,
        }
        /// Hints for which attachments will be accepted.
        #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
        pub struct MediaAttachments {
            /// Contains MIME types that can be uploaded.
            pub supported_mime_types: Vec<String>,
            /// The maximum size of any uploaded image, in bytes.
            pub image_size_limit: i64,
            /// The maximum number of pixels (width times height) for image uploads.
            pub image_matrix_limit: i64,
            /// The maximum size of any uploaded video, in bytes.
            pub video_size_limit: i64,
            /// The maximum frame rate for any uploaded video.
            pub video_frame_rate_limit: i64,
            /// The maximum number of pixels (width times height) for video uploads.
            pub video_matrix_limit: i64,
        }
        /// Limits related to polls.
        #[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
        pub struct Polls {
            /// Each poll is allowed to have up to this many options.
            pub max_options: i64,
            /// Each poll option is allowed to have this many characters.
            pub max_characters_per_option: i64,
            /// The shortest allowed poll duration, in seconds.
            pub min_expiration: i64,
            /// The longest allowed poll duration, in seconds
            pub max_expiration: i64,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_deserialize() {
            let example = r#"{
  "uri":"mastodon.social",
  "title":"Mastodon",
  "short_description":"The original server operated by the Mastodon gGmbH non-profit",
  "description":"",
  "email":"staff@mastodon.social",
  "version":"3.5.3",
  "urls":{
    "streaming_api":"wss://mastodon.social"
  },
  "stats":{
    "user_count":812303,
    "status_count":38151616,
    "domain_count":25255
  },
  "thumbnail":"https://files.mastodon.social/site_uploads/files/000/000/001/original/vlcsnap-2018-08-27-16h43m11s127.png",
  "languages":[
    "en"
  ],
  "registrations":false,
  "approval_required":false,
  "invites_enabled":true,
  "configuration":{
    "statuses":{
      "max_characters":500,
      "max_media_attachments":4,
      "characters_reserved_per_url":23
    },
    "media_attachments":{
      "supported_mime_types":[
        "image/jpeg",
        "image/png",
        "image/gif",
        "image/webp",
        "video/webm",
        "video/mp4",
        "video/quicktime",
        "video/ogg",
        "audio/wave",
        "audio/wav",
        "audio/x-wav",
        "audio/x-pn-wave",
        "audio/vnd.wave",
        "audio/ogg",
        "audio/vorbis",
        "audio/mpeg",
        "audio/mp3",
        "audio/webm",
        "audio/flac",
        "audio/aac",
        "audio/m4a",
        "audio/x-m4a",
        "audio/mp4",
        "audio/3gpp",
        "video/x-ms-asf"
      ],
      "image_size_limit":10485760,
      "image_matrix_limit":16777216,
      "video_size_limit":41943040,
      "video_frame_rate_limit":60,
      "video_matrix_limit":2304000
    },
    "polls":{
      "max_options":4,
      "max_characters_per_option":50,
      "min_expiration":300,
      "max_expiration":2629746
    }
  },
  "contact_account":{
    "id":"1",
    "username":"Gargron",
    "acct":"Gargron",
    "display_name":"Eugen",
    "locked":false,
    "bot":false,
    "discoverable":true,
    "group":false,
    "created_at":"2016-03-16T00:00:00.000Z",
    "note":"\u003cp\u003eFounder, CEO and lead developer \u003cspan class=\"h-card\"\u003e\u003ca href=\"https://mastodon.social/@Mastodon\" class=\"u-url mention\"\u003e@\u003cspan\u003eMastodon\u003c/span\u003e\u003c/a\u003e\u003c/span\u003e, Germany.\u003c/p\u003e",
    "url":"https://mastodon.social/@Gargron",
    "avatar":"https://files.mastodon.social/accounts/avatars/000/000/001/original/dc4286ceb8fab734.jpg",
    "avatar_static":"https://files.mastodon.social/accounts/avatars/000/000/001/original/dc4286ceb8fab734.jpg",
    "header":"https://files.mastodon.social/accounts/headers/000/000/001/original/3b91c9965d00888b.jpeg",
    "header_static":"https://files.mastodon.social/accounts/headers/000/000/001/original/3b91c9965d00888b.jpeg",
    "followers_count":118944,
    "following_count":305,
    "statuses_count":72309,
    "last_status_at":"2022-08-24",
    "emojis":[
      
    ],
    "fields":[
      {
        "name":"Patreon",
        "value":"\u003ca href=\"https://www.patreon.com/mastodon\" target=\"_blank\" rel=\"nofollow noopener noreferrer me\"\u003e\u003cspan class=\"invisible\"\u003ehttps://www.\u003c/span\u003e\u003cspan class=\"\"\u003epatreon.com/mastodon\u003c/span\u003e\u003cspan class=\"invisible\"\u003e\u003c/span\u003e\u003c/a\u003e",
        "verified_at":null
      }
    ]
  },
  "rules":[
    {
      "id":"1",
      "text":"Sexually explicit or violent media must be marked as sensitive when posting"
    },
    {
      "id":"2",
      "text":"No racism, sexism, homophobia, transphobia, xenophobia, or casteism"
    },
    {
      "id":"3",
      "text":"No incitement of violence or promotion of violent ideologies"
    },
    {
      "id":"4",
      "text":"No harassment, dogpiling or doxxing of other users"
    },
    {
      "id":"5",
      "text":"No content illegal in Germany"
    },
    {
      "id":"7",
      "text":"Do not share intentionally false or misleading information"
    }
  ]
}"#;
            let instance: v1::Instance = serde_json::from_str(example).expect("deserialize");
            assert_eq!(instance.uri, "mastodon.social");
        }
    }
}

#[cfg(test)]
mod tests {
    use time::format_description::well_known::Iso8601;

    use super::*;

    #[test]
    fn test_deserialize() {
        let example = r##"{
  "domain": "mastodon.social",
  "title": "Mastodon",
  "version": "4.0.0rc1",
  "source_url": "https://github.com/mastodon/mastodon",
  "description": "The original server operated by the Mastodon gGmbH non-profit",
  "usage": {
    "users": {
      "active_month": 123122
    }
  },
  "thumbnail": {
    "url": "https://files.mastodon.social/site_uploads/files/000/000/001/@1x/57c12f441d083cde.png",
    "blurhash": "UeKUpFxuo~R%0nW;WCnhF6RjaJt757oJodS$",
    "versions": {
      "@1x": "https://files.mastodon.social/site_uploads/files/000/000/001/@1x/57c12f441d083cde.png",
      "@2x": "https://files.mastodon.social/site_uploads/files/000/000/001/@2x/57c12f441d083cde.png"
    }
  },
  "languages": [
    "en"
  ],
  "configuration": {
    "urls": {
      "streaming": "wss://mastodon.social"
    },
    "accounts": {
      "max_featured_tags": 10
    },
    "statuses": {
      "max_characters": 500,
      "max_media_attachments": 4,
      "characters_reserved_per_url": 23
    },
    "media_attachments": {
      "supported_mime_types": [
        "image/jpeg",
        "image/png",
        "image/gif",
        "image/heic",
        "image/heif",
        "image/webp",
        "video/webm",
        "video/mp4",
        "video/quicktime",
        "video/ogg",
        "audio/wave",
        "audio/wav",
        "audio/x-wav",
        "audio/x-pn-wave",
        "audio/vnd.wave",
        "audio/ogg",
        "audio/vorbis",
        "audio/mpeg",
        "audio/mp3",
        "audio/webm",
        "audio/flac",
        "audio/aac",
        "audio/m4a",
        "audio/x-m4a",
        "audio/mp4",
        "audio/3gpp",
        "video/x-ms-asf"
      ],
      "image_size_limit": 10485760,
      "image_matrix_limit": 16777216,
      "video_size_limit": 41943040,
      "video_frame_rate_limit": 60,
      "video_matrix_limit": 2304000
    },
    "polls": {
      "max_options": 4,
      "max_characters_per_option": 50,
      "min_expiration": 300,
      "max_expiration": 2629746
    },
    "translation": {
      "enabled": true
    }
  },
  "registrations": {
    "enabled": false,
    "approval_required": false,
    "message": null
  },
  "contact": {
    "email": "staff@mastodon.social",
    "account": {
      "id": "1",
      "username": "Gargron",
      "acct": "Gargron",
      "display_name": "Eugen 💀",
      "locked": false,
      "bot": false,
      "discoverable": true,
      "group": false,
      "created_at": "2016-03-16T00:00:00.000Z",
      "note": "<p>Founder, CEO and lead developer <span class=\"h-card\"><a href=\"https://mastodon.social/@Mastodon\" class=\"u-url mention\">@<span>Mastodon</span></a></span>, Germany.</p>",
      "url": "https://mastodon.social/@Gargron",
      "avatar": "https://files.mastodon.social/accounts/avatars/000/000/001/original/dc4286ceb8fab734.jpg",
      "avatar_static": "https://files.mastodon.social/accounts/avatars/000/000/001/original/dc4286ceb8fab734.jpg",
      "header": "https://files.mastodon.social/accounts/headers/000/000/001/original/3b91c9965d00888b.jpeg",
      "header_static": "https://files.mastodon.social/accounts/headers/000/000/001/original/3b91c9965d00888b.jpeg",
      "followers_count": 133026,
      "following_count": 311,
      "statuses_count": 72605,
      "last_status_at": "2022-10-31",
      "noindex": false,
      "emojis": [],
      "fields": [
        {
          "name": "Patreon",
          "value": "<a href=\"https://www.patreon.com/mastodon\" target=\"_blank\" rel=\"nofollow noopener noreferrer me\"><span class=\"invisible\">https://www.</span><span class=\"\">patreon.com/mastodon</span><span class=\"invisible\"></span></a>",
          "verified_at": null
        }
      ]
    }
  },
  "rules": [
    {
      "id": "1",
      "text": "Sexually explicit or violent media must be marked as sensitive when posting"
    },
    {
      "id": "2",
      "text": "No racism, sexism, homophobia, transphobia, xenophobia, or casteism"
    },
    {
      "id": "3",
      "text": "No incitement of violence or promotion of violent ideologies"
    },
    {
      "id": "4",
      "text": "No harassment, dogpiling or doxxing of other users"
    },
    {
      "id": "5",
      "text": "No content illegal in Germany"
    },
    {
      "id": "7",
      "text": "Do not share intentionally false or misleading information"
    }
  ]
}"##;
        let subject: Instance = serde_json::from_str(example).expect("deserialize");
        assert_eq!(subject.domain, "mastodon.social");
    }

    #[test]
    fn test_extended_description() {
        let example = r#"{
            "updated_at":"2022-11-03T04:09:07Z",
            "content":"<p>For inquiries not related specifically to the operation of this server, such as press inquiries, please contact <a href=\"mailto:press@joinmastodon.org\">press@joinmastodon.org</a>.</p>\n\n<h2>Funding</h2>\n\n<p>This server is crowdfunded by <a href=\"https://patreon.com/mastodon\">Patreon donations</a>. For a list of sponsors, see <a href=\"https://joinmastodon.org/sponsors\">joinmastodon.org</a>.</p>\n\n<h2>Reporting and moderation</h2>\n\n<p>When reporting accounts, please make sure to include at least a few posts that show rule-breaking behaviour, when applicable. If there is any additional context that might help make a decision, please also include it in the comment. This is especially important when the content is in a language nobody on the moderation team speaks.</p>\n\n<p>We usually handle reports within 24 hours. Please mind that you are not notified when a report you have made has led to a punitive action, and that not all punitive actions are externally visible. For first time offenses, we may opt to delete offending content, escalating to harsher measures on repeat offenses.</p>\n\n<h2>Impressum</h2>\n\n<p>Mastodon gGmbH<br>\nMühlenstraße 8a<br>\n14167 Berlin<br>\nGermany</p>\n\n<p>E-Mail-Adresse: hello@joinmastodon.org</p>\n\n<p>Vertretungsberechtigt: Eugen Rochko (Geschäftsführer)</p>\n\n<p>Umsatzsteuer Identifikationsnummer (USt-ID): DE344258260</p>\n\n<p>Handelsregister<br>\nGeführt bei: Amtsgericht Charlottenburg<br>\nNummer: HRB 230086 B</p>\n"
        }"#;
        let subject: ExtendedDescription = serde_json::from_str(example).expect("deserialize");
        assert_eq!(
            subject.updated_at,
            OffsetDateTime::parse("2022-11-03T04:09:07Z", &Iso8601::PARSING)
                .expect("parse test date")
        );
        assert_eq!(subject.content, "<p>For inquiries not related specifically to the operation of this server, such as press inquiries, please contact <a href=\"mailto:press@joinmastodon.org\">press@joinmastodon.org</a>.</p>\n\n<h2>Funding</h2>\n\n<p>This server is crowdfunded by <a href=\"https://patreon.com/mastodon\">Patreon donations</a>. For a list of sponsors, see <a href=\"https://joinmastodon.org/sponsors\">joinmastodon.org</a>.</p>\n\n<h2>Reporting and moderation</h2>\n\n<p>When reporting accounts, please make sure to include at least a few posts that show rule-breaking behaviour, when applicable. If there is any additional context that might help make a decision, please also include it in the comment. This is especially important when the content is in a language nobody on the moderation team speaks.</p>\n\n<p>We usually handle reports within 24 hours. Please mind that you are not notified when a report you have made has led to a punitive action, and that not all punitive actions are externally visible. For first time offenses, we may opt to delete offending content, escalating to harsher measures on repeat offenses.</p>\n\n<h2>Impressum</h2>\n\n<p>Mastodon gGmbH<br>\nMühlenstraße 8a<br>\n14167 Berlin<br>\nGermany</p>\n\n<p>E-Mail-Adresse: hello@joinmastodon.org</p>\n\n<p>Vertretungsberechtigt: Eugen Rochko (Geschäftsführer)</p>\n\n<p>Umsatzsteuer Identifikationsnummer (USt-ID): DE344258260</p>\n\n<p>Handelsregister<br>\nGeführt bei: Amtsgericht Charlottenburg<br>\nNummer: HRB 230086 B</p>\n");
    }
}
