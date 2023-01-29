use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

use crate::AllowedDomainId;

/// Represents a domain allowed to federate.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_DomainAllow/)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainAllow {
    /// The ID of the DomainAllow in the database.
    pub id: AllowedDomainId,
    /// The domain that is allowed to federate.
    pub domain: String,
    /// When the domain was allowed to federate.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
}
