//! module containing information about a finished report of a user.
use derive_is_enum_variant::is_enum_variant;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

use crate::{account::Account, ReportId, RuleId, StatusId};

/// Reports filed against users and/or statuses, to be taken action on by moderators.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Report/)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Report {
    /// The ID of the report in the database.
    pub id: ReportId,
    /// Whether an action was taken yet.
    pub action_taken: bool,
    /// When an action was taken against the report.
    #[serde(with = "iso8601::option")]
    pub action_taken_at: Option<OffsetDateTime>,
    /// The generic reason for the report.
    pub category: Category,
    /// The reason for the report.
    pub comment: String,
    /// Whether the report was forwarded to a remote domain.
    pub forwarded: bool,
    /// When the report was created.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// IDs of statuses that have been attached to this report for additional
    /// context.
    pub status_ids: Vec<StatusId>,
    /// IDs of the rules that have been cited as a violation by this report.
    #[serde(default)]
    pub rule_ids: Option<Vec<RuleId>>,
    /// The account that was reported.
    pub target_account: Account,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, is_enum_variant)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    /// Malicious, fake, or repetitive content
    Spam,
    /// Violates one or more specific rules
    Violation,
    /// The default (catch-all) category
    Other,
}

impl Default for Category {
    fn default() -> Self {
        Self::Other
    }
}

#[cfg(test)]
mod tests {
    use time::format_description::well_known::Iso8601;

    use super::*;

    #[test]
    fn test_deserialize() {
        let example = r#"{
          "id": "48914",
          "action_taken": false,
          "action_taken_at": null,
          "category": "spam",
          "comment": "Spam account",
          "forwarded": false,
          "created_at": "2022-08-25T09:56:16.763Z",
          "status_ids": [
            "108882889550545820"
          ],
          "rule_ids": null,
          "target_account": {
            "id": "108366849347798387",
            "username": "Baluke",
            "acct": "Baluke",
            "display_name": "Baluke Dental Studios",
            "locked": false,
            "bot": false,
            "discoverable": false,
            "group": false,
            "created_at": "2022-05-26T00:00:00.000Z",
            "note": "<p>Baluke Dental Studios is a full service dental lab offering fabrication, staining, and digital services. Advanced technologies and a meticulous process ensure reduced chair time, lower costs, and better patient outcomes with beautiful smiles. Talk to a representative today.</p><p><a href=\"https://baluke.com/\" target=\"_blank\" rel=\"nofollow noopener noreferrer\"><span class=\"invisible\">https://</span><span class=\"\">baluke.com/</span><span class=\"invisible\"></span></a></p>",
            "url": "https://mastodon.social/@Baluke",
            "avatar": "https://files.mastodon.social/accounts/avatars/108/366/849/347/798/387/original/dbcfe99ed5def0f4.png",
            "avatar_static": "https://files.mastodon.social/accounts/avatars/108/366/849/347/798/387/original/dbcfe99ed5def0f4.png",
            "header": "https://static-cdn.mastodon.social/headers/original/missing.png",
            "header_static": "https://static-cdn.mastodon.social/headers/original/missing.png",
            "followers_count": 0,
            "following_count": 0,
            "statuses_count": 38,
            "last_status_at": "2022-08-25",
            "emojis": [],
            "fields": []
          }
        }"#;
        let subject: Report = serde_json::from_str(example).expect("deserialize");
        assert_eq!(subject.id, ReportId::new("48914"));
        assert!(!subject.action_taken);
        assert!(subject.action_taken_at.is_none());
        assert!(subject.category.is_spam());
        assert_eq!(subject.comment, "Spam account");
        assert!(!subject.forwarded);
        assert_eq!(
            subject.created_at,
            OffsetDateTime::parse("2022-08-25T09:56:16.763Z", &Iso8601::PARSING)
                .expect("created at")
        );
        assert_eq!(subject.status_ids[0], StatusId::new("108882889550545820"));
        assert_eq!(subject.status_ids.len(), 1);
        assert!(subject.rule_ids.is_none());
    }
}
