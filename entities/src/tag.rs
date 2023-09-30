use crate::{conversion, TagId};
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, Date, OffsetDateTime};
use url::Url;

/// Represents a hashtag used within the content of a status.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Tag/)
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Tag {
    /// The value of the hashtag after the `#` sign.
    pub name: String,
    /// A link to the hashtag on the instance.
    pub url: String,
    /// Usage statistics for given days (typically the past week).
    pub history: Vec<History>,
    /// Whether the current token’s authorized user is following this tag.
    ///
    /// `None` when querying without an authenticated token.,
    pub following: Option<bool>,
}

/// Usage statistics for given days (typically the past week).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct History {
    /// UNIX timestamp on midnight of the given day.
    #[serde(with = "conversion::date_from_timestamp")]
    pub day: Date,
    /// The counted usage of the tag within that day.
    #[serde(with = "conversion::string_to::u64")]
    pub uses: u64,
    /// The total of accounts using the tag within that day.
    #[serde(with = "conversion::string_to::u64")]
    pub accounts: u64,
}

/// Represents a hashtag that is featured on a profile.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/FeaturedTag/)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Featured {
    /// The internal ID of the featured tag in the database.
    pub id: TagId,
    /// The name of the hashtag being featured.
    pub name: String,
    /// A link to all statuses by a user that contain this hashtag.
    pub url: Url,
    /// The number of authored statuses containing this hashtag.
    pub statuses_count: u64,
    /// The timestamp of the last authored status containing this hashtag.
    #[serde(with = "iso8601")]
    pub last_status_at: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use time::{Month, format_description::well_known::Iso8601};
    
    use super::*;

    #[test]
    fn test_deserialize() {
        let example = r#"{
          "name": "nowplaying",
          "url": "https://mastodon.social/tags/nowplaying",
          "history": [
            {
              "day": "1574553600",
              "uses": "200",
              "accounts": "31"
            },
            {
              "day": "1574467200",
              "uses": "272",
              "accounts": "39"
            },
            {
              "day": "1574380800",
              "uses": "345",
              "accounts": "40"
            },
            {
              "day": "1574294400",
              "uses": "366",
              "accounts": "46"
            },
            {
              "day": "1574208000",
              "uses": "226",
              "accounts": "32"
            },
            {
              "day": "1574121600",
              "uses": "217",
              "accounts": "42"
            },
            {
              "day": "1574035200",
              "uses": "214",
              "accounts": "34"
            }
          ],
          "following": false
        }"#;
        let subject: Tag = serde_json::from_str(example).unwrap();
        assert_eq!(subject.name, "nowplaying");
        assert_eq!(subject.url, "https://mastodon.social/tags/nowplaying");
        let entry = &subject.history[0];
        assert_eq!(entry.day.year(), 2019);
        assert_eq!(entry.day.month(), Month::November);
        assert_eq!(entry.day.day(), 24);
        assert_eq!(entry.uses, 200);
        assert_eq!(entry.accounts, 31);
        assert_eq!(subject.following, Some(false));
    }

    #[test]
    fn test_featured_tag() {
        let example = r#"{
            "id": "627",
            "name": "nowplaying",
            "url": "https://mastodon.social/@trwnh/tagged/nowplaying",
            "statuses_count": 70,
            "last_status_at": "2022-08-29T12:03:35.061Z"
        }"#;
        let subject: super::Featured = serde_json::from_str(example).expect("deserialize");
        assert_eq!(subject.id, TagId::new("627"));
        assert_eq!(subject.name, "nowplaying");
        assert_eq!(
            subject.url.as_ref(),
            "https://mastodon.social/@trwnh/tagged/nowplaying"
        );
        assert_eq!(subject.statuses_count, 70);
        assert_eq!(
            subject.last_status_at,
            OffsetDateTime::parse("2022-08-29T12:03:35.061Z", &Iso8601::PARSING)
                .expect("parse test time")
        );
    }
}
