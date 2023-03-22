use crate::{
    conversion,
    prelude::{Attachment, Visibility},
    ApplicationId, AttachmentId, StatusId,
};
use isolang::Language;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// Represents a status that will be published at a future scheduled date.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/ScheduledStatus/)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Status {
    /// ID of the scheduled status in the database.
    pub id: StatusId,
    ///
    #[serde(with = "iso8601")]
    pub scheduled_at: OffsetDateTime,
    /// The parameters that were used when scheduling the status, to be used when the status is posted.
    pub params: Params,
    pub media_attachments: Vec<Attachment>,
}

/// The parameters that were used when scheduling the status, to be used when the status is posted.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/ScheduledStatus/#params)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Params {
    /// Text to be used as status content.
    pub text: String,
    /// Poll to be attached to the status.
    pub poll: Option<Poll>,
    /// IDs of the MediaAttachments that will be attached to the status.
    pub media_ids: Option<Vec<AttachmentId>>,
    /// Whether the status will be marked as sensitive.
    pub sensitive: Option<bool>,
    /// The text of the content warning or summary for the status.
    pub spoiler_text: Option<String>,
    /// The language that will be used for the status.
    pub visibility: Option<Visibility>,
    /// ID of the Status that will be replied to.
    pub in_reply_to_id: Option<StatusId>,
    /// The language that will be used for the status.
    pub language: Option<Language>,
    /// ID of the Application that posted the status.
    pub application_id: Option<ApplicationId>,
    /// When the status will be scheduled. This will be null because the status is only scheduled once.
    pub scheduled_at: Option<()>,
    /// Idempotency key to prevent duplicate statuses.
    pub idempotency: Option<String>,
    /// Whether the status should be rate limited .
    pub with_rate_limit: bool,
}

/// Poll to be attached to the status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Poll {
    /// The poll options to be used.
    pub options: Vec<String>,
    /// How many seconds the poll should last before closing.
    #[serde(with = "conversion::string_to::u64")]
    pub expires_in: u64,
    /// Whether the poll allows multiple choices.
    pub multiple: Option<bool>,
    /// Whether the poll should hide total votes until after voting has ended.
    pub hide_totals: Option<bool>,
}

#[cfg(test)]
mod tests {
    use time::format_description::well_known::Iso8601;

    use super::*;

    #[test]
    fn test_get_from_post() {
        let example = r#"{
          "id": "1",
          "scheduled_at": "2022-09-29T00:00:00.000Z",
          "params": {
            "text": "test post",
            "media_ids": null,
            "sensitive": null,
            "spoiler_text": null,
            "visibility": null,
            "language": null,
            "scheduled_at": null,
            "poll": null,
            "idempotency": null,
            "with_rate_limit": false,
            "in_reply_to_id": null,
            "application_id": 3
          },
          "media_attachments": []
        }
        "#;
        let subject: Status = serde_json::from_str(example).expect("deserialize");
        assert_eq!(subject.id, StatusId::new("1"));
        let scheduled_time = OffsetDateTime::parse("2022-09-29T00:00:00.000Z", &Iso8601::PARSING)
            .expect("parse scheduled time");
        assert_eq!(subject.scheduled_at, scheduled_time);
        let params = subject.params;
        assert_eq!(params.text, "test post");
        assert!(params.media_ids.is_none());
        assert!(params.sensitive.is_none());
        assert!(params.spoiler_text.is_none());
        assert!(params.visibility.is_none());
        assert!(params.language.is_none());
        assert!(params.scheduled_at.is_none());
        assert!(params.poll.is_none());
        assert!(params.idempotency.is_none());
        assert!(!params.with_rate_limit);
        assert_eq!(
            params.application_id.expect("application_id"),
            ApplicationId::new("3")
        );
        assert!(subject.media_attachments.is_empty());
    }

    #[test]
    fn test_get() {
        let example = r#"{
            "id": "1",
            "scheduled_at": "2022-09-29T00:00:00.000Z",
            "params": {
              "poll": null,
              "text": "test post",
              "language": null,
              "media_ids": null,
              "sensitive": null,
              "visibility": null,
              "idempotency": null,
              "scheduled_at": null,
              "spoiler_text": null,
              "application_id": 3,
              "in_reply_to_id": null,
              "with_rate_limit": false
            },
            "media_attachments": []
        }"#;
        let subject: Status = serde_json::from_str(example).expect("deserialize");
        assert_eq!(subject.id, StatusId::new("1"));
        let scheduled_time = OffsetDateTime::parse("2022-09-29T00:00:00.000Z", &Iso8601::PARSING)
            .expect("parse scheduled time");
        assert_eq!(subject.scheduled_at, scheduled_time);
        let params = subject.params;
        assert_eq!(params.text, "test post");
        assert!(params.media_ids.is_none());
        assert!(params.sensitive.is_none());
        assert!(params.spoiler_text.is_none());
        assert!(params.visibility.is_none());
        assert!(params.language.is_none());
        assert!(params.scheduled_at.is_none());
        assert!(params.poll.is_none());
        assert!(params.idempotency.is_none());
        assert!(!params.with_rate_limit);
        assert_eq!(
            params.application_id.expect("application_id"),
            ApplicationId::new("3")
        );
        assert!(subject.media_attachments.is_empty());
    }
}
