/// Create a new email domain block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddEmailDomainBlockRequest {
    /// The email domain to block signups from with.
    pub domain: String,
}

impl AddEmailDomainBlockRequest {
    /// Create a block for an email domain.
    pub fn new<T>(domain: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            domain: domain.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_add_request() {
        let request = AddEmailDomainBlockRequest::new("example.org");
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(ser, r#"{"domain":"example.org"}"#);
    }
}
