use crate::entities::{report::Category, RuleId};
use derive_builder::Builder;
use mastodon_async_derive::RequestBuilder;
use serde_with::{serde_as, skip_serializing_none};

/// Change metadata for a report.
/// https://docs.joinmastodon.org/methods/admin/reports/#path-parameters-1
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Builder, RequestBuilder)]
#[builder(
    derive(Debug, PartialEq),
    custom_constructor,
    build_fn(private, name = "try_build"),
    setter(into, strip_option)
)]
pub struct UpdateReportRequest {
    /// Updated category of the report.
    #[builder(default)]
    pub category: Option<Category>,
    /// Updated rule IDs for [`Category::Violation`] reports.
    #[builder(default)]
    pub rule_ids: Option<Vec<RuleId>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_update_request() {
        let request = UpdateReportRequest::builder()
            .category(Category::Spam)
            .build();
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(ser, r#"{"category":"spam"}"#);
    }
}
