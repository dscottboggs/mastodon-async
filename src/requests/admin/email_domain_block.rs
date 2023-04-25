use mastodon_async_derive::request_builder;

/// Create a new email domain block.
#[request_builder]
pub struct AddEmailDomainBlockRequest {
    /// The email domain to block signups from.
    pub domain: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_add_request() {
        let request = AddEmailDomainBlockRequest::builder("example.org").build();
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(ser, r#"{"domain":"example.org"}"#);
    }
}
