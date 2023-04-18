use crate::entities::admin::domain::BlockSeverity;
use derive_builder::Builder;
use serde_with::skip_serializing_none;

/// Create a new domain allow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddDomainAllowRequest {
    /// The domain to allow federation with.
    pub domain: String,
}

impl AddDomainAllowRequest {
    /// Create a domain allow for a domain.
    pub fn new<T>(domain: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            domain: domain.into(),
        }
    }
}

/// Create a new domain block or update an existing one.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Builder)]
#[builder(
    derive(Debug, PartialEq),
    custom_constructor,
    build_fn(private, name = "try_build"),
    setter(into, strip_option)
)]
pub struct AddDomainBlockRequest {
    /// The domain to block federation with.
    #[builder(private)]
    pub domain: String,
    /// Policy to be applied to the domain.
    #[builder(default)]
    pub severity: Option<BlockSeverity>,
    /// Whether media attachments should be rejected.
    #[builder(default)]
    pub reject_media: Option<bool>,
    /// Whether media attachments should be rejected.
    #[builder(default)]
    pub reject_reports: Option<bool>,
    /// A private note about this domain block, visible only to admins.
    #[builder(default)]
    pub private_comment: Option<String>,
    /// A public note about this domain block, optionally shown on the about page.
    #[builder(default)]
    pub public_comment: Option<String>,
    /// Whether to partially censor the domain when shown in public.
    #[builder(default)]
    pub obfuscate: Option<bool>,
}

pub use AddDomainBlockRequest as UpdateDomainBlockRequest;

impl AddDomainBlockRequest {
    /// Start building a form for creating or updating a domain block.
    pub fn builder<T>(domain: T) -> AddDomainBlockRequestBuilder
    where
        T: Into<String>,
    {
        let mut builder = AddDomainBlockRequestBuilder::create_empty();
        builder.domain(domain);
        builder
    }
}

impl AddDomainBlockRequestBuilder {
    /// Build the form for creating or updating a domain block.
    pub fn build(&self) -> AddDomainBlockRequest {
        self.try_build()
            .expect("One or more required fields are missing!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_add_allow_request() {
        let request = AddDomainAllowRequest::new("example.org");
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
