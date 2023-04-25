use crate::entities::{report::Category, RuleId};
use mastodon_async_derive::request_builder;
use serde_with::serde_as;

/// Change metadata for a report.
/// https://docs.joinmastodon.org/methods/admin/reports/#path-parameters-1
#[serde_as]
#[request_builder]
pub struct UpdateReportRequest {
    /// Updated category of the report.
    pub category: Option<Category>,
    /// Updated rule IDs for [`Category::Violation`] reports.
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
