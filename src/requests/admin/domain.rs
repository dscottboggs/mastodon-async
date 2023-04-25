use crate::entities::admin::domain::BlockSeverity;
use mastodon_async_derive::request_builder;

/// Create a new domain allow.
#[request_builder]
pub struct AddDomainAllowRequest {
    /// The domain to allow federation with.
    pub domain: String,
}

/// Create a new domain block or update an existing one.
#[request_builder]
pub struct AddDomainBlockRequest {
    /// The domain to block federation with.
    pub domain: String,
    /// Policy to be applied to the domain.
    pub severity: Option<BlockSeverity>,
    /// Whether media attachments should be rejected.
    pub reject_media: Option<bool>,
    /// Whether media attachments should be rejected.
    pub reject_reports: Option<bool>,
    /// A private note about this domain block, visible only to admins.
    pub private_comment: Option<String>,
    /// A public note about this domain block, optionally shown on the about page.
    pub public_comment: Option<String>,
    /// Whether to partially censor the domain when shown in public.
    pub obfuscate: Option<bool>,
}

pub use AddDomainBlockRequest as UpdateDomainBlockRequest;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_add_allow_request() {
        let request = AddDomainAllowRequest::builder("example.org").build();
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(ser, r#"{"domain":"example.org"}"#);
    }

    #[test]
    fn test_serialize_add_block_request() {
        let request = AddDomainBlockRequest::builder("example.org")
            .severity(BlockSeverity::Silence)
            .reject_media(true)
            .public_comment("public comment")
            .build();
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(
            ser,
            r#"{"domain":"example.org","severity":"silence","reject_media":true,"public_comment":"public comment"}"#
        );
    }
}
