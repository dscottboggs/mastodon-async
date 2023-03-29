use derive_builder::Builder;
use mastodon_async_entities::{report::Category, AccountId, RuleId, StatusId};
use serde_with::skip_serializing_none;

/// Form used to create a report
///
/// // Example
///
/// ```
/// use mastodon_async::{entities::{AccountId, report::Category}, requests::AddReportRequest};
/// let request = AddReportRequest::builder(AccountId::new("666")).category(Category::Spam).build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Builder)]
#[builder(
    custom_constructor,
    build_fn(private, name = "try_build"),
    setter(into, strip_option)
)]
pub struct AddReportRequest {
    /// The account being reported.
    #[builder(private)]
    pub account_id: AccountId,
    /// Attach statuses to the report to provide additional context.
    #[builder(default)]
    pub status_ids: Option<Vec<StatusId>>,
    /// The reason for the report.
    #[builder(default)]
    pub comment: Option<String>,
    /// If the account is remote, should the report be forwarded to the remote admin?
    #[builder(default)]
    pub forward: Option<bool>,
    /// Machine-readable category of the report.
    #[builder(default)]
    pub category: Option<Category>,
    /// Rules broken by a [`Category::Violation`] report.
    #[builder(default)]
    pub rule_ids: Option<Vec<RuleId>>,
}

impl AddReportRequest {
    /// Start building a form for creating a report.
    pub fn builder(account_id: AccountId) -> AddReportRequestBuilder {
        let mut builder = AddReportRequestBuilder::create_empty();
        builder.account_id(account_id);
        builder
    }
}

impl AddReportRequestBuilder {
    /// Build the form for creating a report.
    pub fn build(&self) -> AddReportRequest {
        self.try_build()
            .expect("One or more required fields are missing!")
    }
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
