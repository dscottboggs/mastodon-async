use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime, Date};

use crate::{conversion, EmailDomainBlockId};

/// Represents an email domain that cannot be used to sign up.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_EmailDomainBlock/)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailDomainBlock {
    /// The ID of the domain in the database
    pub id: EmailDomainBlockId,
    /// The email domain that is not allowed to be used for signups.
    pub domain: String,
    /// When the email domain was disallowed from signups.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// Usage statistics for given days (typically the past week).
    pub history: Vec<History>,
}

/// Usage history for a given day
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct History {
    #[serde(with = "conversion::date_from_timestamp")]
    pub day: Date,
    /// The counted accounts signup attempts using that email domain within that day.
    #[serde(with = "conversion::string_to_u64")]
    pub accounts: u64,
    /// The counted IP signup attempts of that email domain within that day.
    #[serde(with = "conversion::string_to_u64")]
    pub uses: u64,
}

#[cfg(test)]
mod tests {
    use time::Month;

    use super::*;

    #[test]
    fn test_example() {
        let example = r#"{
          "id": "1",
          "domain": "foo",
          "created_at": "2022-11-16T06:09:36.176Z",
          "history": [
            {
              "day": "1668556800",
              "accounts": "0",
              "uses": "0"
            },
            {
              "day": "1668470400",
              "accounts": "0",
              "uses": "0"
            },
            {
              "day": "1668384000",
              "accounts": "0",
              "uses": "0"
            },
            {
              "day": "1668297600",
              "accounts": "0",
              "uses": "0"
            },
            {
              "day": "1668211200",
              "accounts": "0",
              "uses": "0"
            },
            {
              "day": "1668124800",
              "accounts": "0",
              "uses": "0"
            },
            {
              "day": "1668038400",
              "accounts": "0",
              "uses": "0"
            }
          ]
        }"#;
        let subject: EmailDomainBlock = serde_json::from_str(example).unwrap();
        assert_eq!(subject.id, EmailDomainBlockId::new("1"));
        assert_eq!(subject.domain, "foo");
        for (i, entry) in subject.history.iter().enumerate() {
            assert_eq!(entry.accounts, 0);
            assert_eq!(entry.uses, 0);
            assert_eq!(entry.day.year(), 2022);
            assert_eq!(entry.day.month(), Month::November);
            assert_eq!(entry.day.day(), (16 - i) as u8);
        }
    }
}