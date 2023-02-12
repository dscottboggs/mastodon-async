use isolang::Language;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use time::{serde::iso8601, OffsetDateTime};

use crate::{account::Role, prelude::AccountId};

/// Admin-level information about a given account.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_Account/)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Account {
    /// The ID of the account in the database.
    pub id: AccountId,
    /// The username of the account.
    pub username: String,
    /// The domain of the account, if it is remote.
    pub domain: Option<String>,
    /// When the account was first discovered.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// The email address associated with the account.
    pub email: String,
    /// The reason given when requesting an invite (for instances that require
    /// manual approval of registrations)
    pub invite_request: Option<String>,
    /// The IP address last used to login to this account.
    pub ip: Option<String>,
    /// All known IP addresses associated with this account.
    pub ips: Vec<Ip>,
    /// The current role of the account.
    pub role: Role,
    /// Whether the account has confirmed their email address.
    pub confirmed: bool,
    /// Whether the account is currently suspended.
    pub suspended: bool,
    /// Whether the account is currently silenced.
    pub silenced: bool,
    /// Whether the account is currently disabled.
    pub disabled: bool,
    /// Whether the account is currently approved.
    pub approved: bool,
    /// The locale of the account.
    pub locale: Option<Language>,
    /// User-level information about the account.
    pub account: crate::account::Account,
    /// The ID of the Application that created this account, if applicable.
    pub created_by_application_id: Option<String>,
    /// The ID of the Account that invited this user, if applicable.
    pub invited_by_account_id: Option<AccountId>,
}

/// Represents an IP address associated with a user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ip {
    /// The IP address.
    pub ip: IpAddr,
    /// The timestamp of when the IP address was last used for this account.
    #[serde(with = "iso8601")]
    pub used_at: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use crate::prelude::AccountId;

    use super::*;

    #[test]
    fn test_deserialize_example() {
        let data = r##"{
  "id": "108965278956942133",
  "username": "admin",
  "domain": null,
  "created_at": "2022-09-08T23:03:26.762Z",
  "email": "admin@mastodon.local",
  "ip": "192.168.42.1",
  "role": {
    "id": 3,
    "name": "Owner",
    "color": "",
    "position": 1000,
    "permissions": 1,
    "highlighted": true,
    "created_at": "2022-09-08T22:48:07.983Z",
    "updated_at": "2022-09-08T22:48:07.983Z"
  },
  "confirmed": true,
  "suspended": false,
  "silenced": false,
  "disabled": false,
  "approved": true,
  "locale": null,
  "invite_request": null,
  "ips": [
    {
      "ip": "192.168.42.1",
      "used_at": "2022-09-15T01:38:58.851Z"
    }
  ],
  "account": {
    "id": "108965278956942133",
    "username": "admin",
    "acct": "admin",
    "display_name": "",
    "locked": false,
    "bot": false,
    "discoverable": null,
    "group": false,
    "created_at": "2022-09-08T00:00:00.000Z",
    "note": "",
    "url": "http://mastodon.local/@admin",
    "avatar": "http://mastodon.local/avatars/original/missing.png",
    "avatar_static": "http://mastodon.local/avatars/original/missing.png",
    "header": "http://mastodon.local/headers/original/missing.png",
    "header_static": "http://mastodon.local/headers/original/missing.png",
    "followers_count": 0,
    "following_count": 0,
    "statuses_count": 0,
    "last_status_at": null,
    "emojis": [],
    "fields": []
  }
}"##;
        let account: Account = serde_json::from_str(data).expect("deserialize");
        assert_eq!(account.id, AccountId::new("108965278956942133"));
        let ip = &account.ips[0];
        assert_eq!(ip.ip.to_string(), "192.168.42.1");
    }
}
