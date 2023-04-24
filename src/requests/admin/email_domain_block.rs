use derive_builder::Builder;
use mastodon_async_derive::RequestBuilder;

/// Create a new email domain block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Builder, RequestBuilder)]
#[builder(
    derive(Debug, PartialEq),
    custom_constructor,
    build_fn(private, name = "try_build"),
    setter(into, strip_option)
)]
pub struct AddEmailDomainBlockRequest {
    /// The email domain to block signups from.
    #[builder(private)]
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
