use crate::{DomainAllowId, DomainBlockId};
use derive_is_enum_variant::is_enum_variant;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// Represents a domain allowed to federate.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_DomainAllow/)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Allow {
    /// The ID of the DomainAllow in the database.
    pub id: DomainAllowId,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, is_enum_variant)]
#[serde(rename_all = "lowercase")]
pub enum BlockSeverity {
    /// Account statuses from this domain will be hidden by default
    Silence,
    /// All incoming data from this domain will be rejected
    Suspend,
    /// Do nothing. Allows for rejecting media or reports
    Noop,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_block_example() {
        let example = r#"{
          "id": "1",
          "domain": "example.com",
          "created_at": "2022-11-16T08:15:34.238Z",
          "severity": "noop",
          "reject_media": false,
          "reject_reports": false,
          "private_comment": null,
          "public_comment": null,
          "obfuscate": false
        }"#;
        let subject: Block = serde_json::from_str(example).unwrap();
        assert_eq!(subject.id, DomainBlockId::new("1"));
        assert_eq!(subject.domain, "example.com");
        assert!(subject.severity.is_noop());
        assert!(!subject.reject_media);
        assert!(!subject.reject_reports);
        assert!(subject.private_comment.is_none());
        assert!(subject.public_comment.is_none());
        assert!(!subject.obfuscate);
    }

    #[test]
    fn test_domain_allow_example() {
        let example = r#"{
        	"id": "1",
        	"domain": "mastodon.social",
        	"created_at": "2022-09-14T21:23:02.755Z"
        }"#;
        let subject: Allow = serde_json::from_str(example).unwrap();
        assert_eq!(subject.id, AllowDomainId::new("1"));
        assert_eq!(subject.domain, "mastodon.social");
    }
}
