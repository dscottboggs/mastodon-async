use serde::Serialize;
use serde_with::{hex::Hex, serde_as, skip_serializing_none};

/// Form to create a new canonical email block.
/// Either the original email or the hash can be submitted.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddCanonicalEmailBlockRequest {
    /// An email to canonicalize, hash, and block.
    pub email: Option<String>,
    /// A pre-hashed email.
    #[serde_as(as = "Option<Hex>")]
    pub canonical_email_hash: Option<Vec<u8>>,
}

impl AddCanonicalEmailBlockRequest {
    /// Create a block for an email address.
    pub fn from_email<T>(email: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            email: Some(email.into()),
            canonical_email_hash: None,
        }
    }

    /// Create a block for a pre-hashed email address.
    pub fn from_canonical_email_hash<T>(canonical_email_hash: T) -> Self
    where
        T: Into<Vec<u8>>,
    {
        Self {
            email: None,
            canonical_email_hash: Some(canonical_email_hash.into()),
        }
    }
}

/// Test an email against existing canonical email blocks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TestCanonicalEmailBlocksRequest {
    email: String,
}

impl TestCanonicalEmailBlocksRequest {
    /// Create a test request for a given email.
    pub fn new<T>(email: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            email: email.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_add_request_email() {
        let request = AddCanonicalEmailBlockRequest::from_email("horrible.doll@example.org");
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(ser, r#"{"email":"horrible.doll@example.org"}"#);
    }

    #[test]
    fn test_serialize_add_request_hash() {
        let request =
            AddCanonicalEmailBlockRequest::from_canonical_email_hash([0x12, 0x34, 0x56, 0x78]);
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(ser, r#"{"canonical_email_hash":"12345678"}"#);
    }

    #[test]
    fn test_serialize_test_request() {
        let request = TestCanonicalEmailBlocksRequest::new("horrible.doll@example.org");
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(ser, r#"{"email":"horrible.doll@example.org"}"#);
    }
}
