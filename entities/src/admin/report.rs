use crate::{admin, instance::Rule, report, status::Status, ReportId};
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// Admin-level information about a filed report.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_Report/)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Report {
    /// The ID of the report in the database.
    pub id: ReportId,
    /// Whether an action was taken to resolve this report.
    pub action_taken: bool,
    /// When an action was taken, if this report is currently resolved.
    #[serde(with = "iso8601::option")]
    pub action_taken_at: Option<OffsetDateTime>,
    /// The category under which the report is classified.
    pub category: report::Category,
    /// An optional reason for reporting.
    pub comment: String,
    /// Whether a report was forwarded to a remote instance.
    pub forwarded: bool,
    /// The time the report was filed.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// The time of last action on this report.
    #[serde(with = "iso8601")]
    pub updated_at: OffsetDateTime,
    /// The account which filed the report.
    pub account: admin::Account,
    /// The account being reported.
    pub target_account: admin::Account,
    /// The account of the moderator assigned to this report.
    pub assigned_account: Option<admin::Account>,
    /// The account of the moderator who handled the report.
    pub action_taken_by_account: Option<admin::Account>,
    /// Statuses attached to the report, for context.
    pub statuses: Vec<Status>,
    /// Rules attached to the report, for context.
    pub rules: Vec<Rule>,
}
