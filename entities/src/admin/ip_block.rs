use crate::IpBlockId;
use derive_is_enum_variant::is_enum_variant;
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// Represents an IP address range that cannot be used to sign up.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_IpBlock/)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IpBlock {
    /// The ID of the IP block in the database.
    pub id: IpBlockId,
    /// The IP address range that is not allowed to federate.
    pub ip: IpNet,
    /// The policy associated with this IP block.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, is_enum_variant)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// Any signup from this IP range will create a pending account
    SignUpRequiresApproval,
    /// Any signup from this IP range will be rejected
    SignUpBlock,
    /// Any activity from this IP range will be rejected entirely
    NoAccess,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_block_example() {
        let example = r#"{
          "id": "1",
          "ip": "8.8.8.8/32",
          "severity": "no_access",
          "comment": "",
          "created_at": "2022-11-16T07:22:00.501Z",
          "expires_at": null
        }"#;
        let subject: IpBlock = serde_json::from_str(example).unwrap();
        assert_eq!(subject.id, DomainBlockId::new("1"));
        assert_eq!(subject.ip, "8.8.8.8/32".parse().unwrap());
        assert!(subject.severity.is_no_access());
        assert!(subject.comment.is_empty());
        assert!(subject.expires_at.is_none());
    }
}
