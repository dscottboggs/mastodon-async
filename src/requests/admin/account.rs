use crate::entities::{ReportId, WarningPresetId};
use derive_is_enum_variant::is_enum_variant;
use mastodon_async_derive::request_builder;

/// Form used to perform an admin action on an account and resolve any open reports
///
/// // Example
///
/// ```
/// use mastodon_async::requests::admin::{AccountAction, AccountActionRequest};
/// let request = AccountActionRequest::builder(AccountAction::Silence).text("Hush now").build();
/// ```
#[request_builder]
pub struct AccountActionRequest {
    /// The type of action to be taken.
    #[serde(rename = "type")]
    pub action: AccountAction,
    /// The ID of an associated report that caused this action to be taken.
    pub report_id: Option<ReportId>,
    /// The ID of a preset warning.
    pub warning_preset_id: Option<WarningPresetId>,
    /// Additional clarification for why this action was taken.
    pub text: Option<String>,
    /// Should an email be sent to the user with the above information?
    pub send_email_notification: Option<bool>,
}

/// Action to be performed on the account.
/// https://docs.joinmastodon.org/methods/admin/accounts/#form-data-parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, is_enum_variant)]
#[serde(rename_all = "snake_case")]
pub enum AccountAction {
    /// No action. Can be used to resolve any open reports against the account.
    None,
    /// Force the account's statuses to be marked as containing sensitive media.
    Sensitive,
    /// Prevent the account from logging in.
    Disable,
    /// Silence the account.
    Silence,
    /// Suspend the account.
    Suspend,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_action_request() {
        let request = AccountActionRequest::builder(AccountAction::Suspend)
            .report_id(ReportId::new("666"))
            .text("you know what you did")
            .build();
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(
            ser,
            r#"{"type":"suspend","report_id":"666","text":"you know what you did"}"#
        );
    }
}
