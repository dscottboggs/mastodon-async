use crate::conversion;
use serde::{Deserialize, Serialize};
use time::Date;

/// Represents a hashtag used within the content of a status.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Tag/)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    /// The value of the hashtag after the `#` sign.
    pub name: String,
    /// A link to the hashtag on the instance.
    pub url: String,
    /// Usage statistics for given days (typically the past week).
    pub history: Vec<History>,
    /// Whether the current token’s authorized user is following this tag.
    ///
    /// `None` when querying without an authenticated token.
    pub following: Option<bool>,
}

/// Usage statistics for given days (typically the past week).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct History {
    /// UNIX timestamp on midnight of the given day.
    #[serde(with = "conversion::date_from_timestamp")]
    pub day: Date,
    /// The counted usage of the tag within that day.
    #[serde(with = "conversion::string_to_u64")]
    pub uses: u64,
    /// The total of accounts using the tag within that day.
    #[serde(with = "conversion::string_to_u64")]
    pub accounts: u64,
}

#[cfg(test)]
mod tests {
    use time::Month;

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
}
