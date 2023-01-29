use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

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
    #[serde(with = "iso8601")]
    pub day: OffsetDateTime,
    /// The counted accounts signup attempts using that email domain within that day.
    #[serde(with = "conversion::string_to_u64")]
    pub accounts: u64,
    /// The counted IP signup attempts of that email domain within that day.
    #[serde(with = "conversion::string_to_u64")]
    pub uses: u64,
}
