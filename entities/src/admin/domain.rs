use is_variant::IsVariant;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

use crate::{AllowDomainId, DomainBlockId};

/// Represents a domain allowed to federate.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_DomainAllow/)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Allow {
    /// The ID of the DomainAllow in the database.
    pub id: AllowDomainId,
    /// The domain that is allowed to federate.
    pub domain: String,
    /// When the domain was allowed to federate.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
}

/// Represents a domain limited from federating.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_DomainBlock/)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    /// The ID of the DomainBlock in the database.
    pub id: DomainBlockId,
    /// The domain that is not allowed to federate.
    pub domain: String,
    /// When the domain was blocked from federating.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// The policy to be applied by this domain block.
    pub severity: BlockSeverity,
    /// Whether to reject media attachments from this domain
    pub reject_media: bool,
    /// Whether to reject reports from this domain
    pub reject_reports: bool,
    /// A private comment
    pub private_comment: Option<String>,
    /// A public comment
    pub public_comment: Option<String>,
    /// Whether to obfuscate public displays of this domain block
    pub obfuscate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, IsVariant)]
#[serde(rename_all = "lowercase")]
pub enum BlockSeverity {
    /// Account statuses from this domain will be hidden by default
    Silence,
    /// All incoming data from this domain will be rejected
    Suspend,
    /// Do nothing. Allows for rejecting media or reports
    Noop,
}
