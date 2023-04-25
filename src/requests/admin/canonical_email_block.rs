use mastodon_async_derive::request_builder;
use serde::Serialize;
use serde_with::{hex::Hex, serde_as};

/// Form to create a new canonical email block.
/// Either the original email or the hash can be submitted.
#[serde_as]
#[request_builder]
pub struct AddCanonicalEmailBlockRequest {
    /// An email to canonicalize, hash, and block.
    pub email: Option<String>,
    /// A pre-hashed email.
    #[serde_as(as = "Option<Hex>")]
    pub canonical_email_hash: Option<Vec<u8>>,
}

/// Test an email against existing canonical email blocks.
#[request_builder]
pub struct TestCanonicalEmailBlocksRequest {
    /// The email to test.
    pub email: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_add_request_email() {
        let request = AddCanonicalEmailBlockRequest::builder()
            .email("horrible.doll@example.org")
            .build();
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(ser, r#"{"email":"horrible.doll@example.org"}"#);
    }

    #[test]
    fn test_serialize_add_request_hash() {
        let request = AddCanonicalEmailBlockRequest::builder()
            .canonical_email_hash([0x12, 0x34, 0x56, 0x78])
            .build();
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(ser, r#"{"canonical_email_hash":"12345678"}"#);
    }

    #[test]
    fn test_serialize_test_request() {
        let request = TestCanonicalEmailBlocksRequest::builder("horrible.doll@example.org").build();
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(ser, r#"{"email":"horrible.doll@example.org"}"#);
    }
}
