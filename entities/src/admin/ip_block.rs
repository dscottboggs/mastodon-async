use crate::DomainBlockId;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// Represents an IP address range that cannot be used to sign up.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_IpBlock/)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IpBlock {
    /// The ID of the DomainBlock in the database.
    pub id: DomainBlockId,
    /// The IP address range that is not allowed to federate.
    pub ip: String,
    /// The associated policy with this IP block.
    pub severity: Severity,
    /// The recorded reason for this IP block.
    pub comment: String,
    /// When the IP block was created.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// When the IP block will expire.
    #[serde(with = "iso8601::option")]
    pub expires_at: Option<OffsetDateTime>,
}

/// The associated policy with some IP block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Any signup from this IP range will create a pending account
    SignUpRequiresApproval,
    /// Any signup from this IP range will be rejected
    SignUpBlock,
    /// Any activity from this IP range will be rejected entirely
    NoAccess,
}
