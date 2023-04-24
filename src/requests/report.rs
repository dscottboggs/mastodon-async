use mastodon_async_derive::request_builder;
use mastodon_async_entities::{report::Category, AccountId, RuleId, StatusId};

/// Form used to create a report
///
/// // Example
///
/// ```
/// use mastodon_async::{entities::{AccountId, report::Category}, requests::AddReportRequest};
/// let request = AddReportRequest::builder(AccountId::new("666")).category(Category::Spam).build();
/// ```
#[request_builder]
pub struct AddReportRequest {
    /// The account being reported.
    pub account_id: AccountId,
    /// Attach statuses to the report to provide additional context.
    pub status_ids: Option<Vec<StatusId>>,
    /// The reason for the report.
    pub comment: Option<String>,
    /// If the account is remote, should the report be forwarded to the remote admin?
    pub forward: Option<bool>,
    /// Machine-readable category of the report.
    pub category: Option<Category>,
    /// Rules broken by a [`Category::Violation`] report.
    pub rule_ids: Option<Vec<RuleId>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_request() {
        let request = AddReportRequest::builder(AccountId::new("666"))
            .category(Category::Spam)
            .build();
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(ser, r#"{"account_id":"666","category":"spam"}"#);
    }
}
