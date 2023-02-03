use derive_builder::Builder;
use isolang::Language;
use serde::Serialize;

use crate::{prelude::Visibility, AttachmentId};

/// Represents a post that can be sent to the POST /api/v1/status endpoint
///
/// See also [the API documentation](https://docs.joinmastodon.org/methods/statuses/#form-data-parameters)
#[derive(Debug, Builder, Default, Clone, Serialize, PartialEq, Eq)]
#[builder(build_fn(error = "crate::error::Error"))]
pub struct NewStatus {
    /// The text content of the status. If media_ids is provided, this becomes
    /// optional. Attaching a poll is optional while status is provided.
    ///
    /// Note that this means there is at this time no check provided by this
    /// type to ensure that this value is set when it is required by the API,
    /// and an APIError should be expected from [`crate::Mastodon::new_status()`]
    /// in this case.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub status: Option<String>,
    /// ID of the status being replied to, if status is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub in_reply_to_id: Option<String>,
    /// Include Attachment IDs to be attached as media. If provided, status
    /// becomes optional, and poll cannot be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub media_ids: Option<Vec<AttachmentId>>,
    /// Mark status and attached media as sensitive? Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sensitive: Option<bool>,
    /// Text to be shown as a warning or subject before the actual content.
    /// Statuses are generally collapsed behind this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub spoiler_text: Option<String>,
    /// Sets the visibility of the posted status to public, unlisted, private, direct.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub visibility: Option<Visibility>,
    /// ISO 639 language code for this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub language: Option<Language>,
    /// Ignored by Mastodon servers, sets the content type for the status.
    /// Mastodon "toots" are always `text/plain`, regardless of this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub content_type: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use isolang::Language;
    use serde_json;

    #[test]
    fn test_new() {
        let s = NewStatusBuilder::default()
            .status("a status")
            .build()
            .expect("Couldn't build status");
        let expected = NewStatus {
            status: Some("a status".to_string()),
            in_reply_to_id: None,
            media_ids: None,
            sensitive: None,
            spoiler_text: None,
            visibility: None,
            language: None,
            content_type: None,
        };
        assert_eq!(s, expected);
    }

    #[test]
    fn test_default_visibility() {
        let v: Visibility = Default::default();
        assert_eq!(v, Visibility::Public);
    }

    #[test]
    fn test_serialize_visibility() {
        assert_eq!(
            serde_json::to_string(&Visibility::Direct).expect("couldn't serialize visibility"),
            "\"direct\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&Visibility::Private).expect("couldn't serialize visibility"),
            "\"private\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&Visibility::Unlisted).expect("couldn't serialize visibility"),
            "\"unlisted\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&Visibility::Public).expect("couldn't serialize visibility"),
            "\"public\"".to_string()
        );
    }

    #[test]
    fn test_serialize_status() {
        let status = NewStatusBuilder::default()
            .status("a status")
            .build()
            .expect("Couldn't build status");
        assert_eq!(
            serde_json::to_string(&status).expect("Couldn't serialize status"),
            "{\"status\":\"a status\"}".to_string()
        );

        let status = NewStatusBuilder::default()
            .status("a status")
            .language(Language::Eng)
            .build()
            .expect("Couldn't build status");
        assert_eq!(
            serde_json::to_string(&status).expect("Couldn't serialize status"),
            "{\"status\":\"a status\",\"language\":\"eng\"}"
        );
    }
}
