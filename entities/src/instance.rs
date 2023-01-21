//! Module containing everything related to an instance.
use serde::{Deserialize, Serialize};

use super::account::Account;

/// A struct containing info of an instance.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Instance {
    /// URI of the current instance
    pub uri: String,
    /// The instance's title.
    pub title: String,
    /// A description for the instance.
    pub description: String,
    /// An email address which can be used to contact the
    /// instance administrator.
    pub email: String,
    /// The Mastodon version used by instance.
    pub version: String,
    /// Urls to the streaming api.
    pub urls: Option<StreamingApi>,
    /// Stats about the instance.
    pub stats: Option<Stats>,
    /// Thumbnail of the server image.
    pub thumbnail: Option<String>,
    /// List of languages used on the server.
    pub languages: Option<Vec<String>>,
    /// Contact account for the server.
    pub contact_account: Option<Account>,
    /// The maximum number of characters allowed in a status
    pub max_toot_chars: Option<u32>,
}

/// Object containing url for streaming api.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct StreamingApi {
    /// Url for streaming API, typically a `wss://` url.
    pub streaming_api: String,
}

/// Statistics about the Mastodon instance.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub struct Stats {
    user_count: u64,
    status_count: u64,
    domain_count: u64,
}

/// Rules of an instance
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Rule {
    /// An identifier for the rule.
    pub id: String,
    /// The rule to be followed.
    pub text: String,
}

/// An instance-level domain block.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DomainBlock {
    /// URI of the domain in question
    pub domain: String,
    /// Digest
    pub digest: String,
    /// Severity of the block
    pub severity: String,
    /// Admin's public comment.
    pub comment: String,
}

/// Weekly activity on an instance
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Activity {
    /// UNIX Timestamp at midnight at the first day of the week.
    pub week: String,
    /// The number of Statuses created since the week began (cast from an integer)
    pub statuses: String,
    /// The number of user logins since the week began (cast from an integer)
    pub logins: String,
    /// The number of user registrations since the week began (cast from an integer)
    pub registrations: String,
}
