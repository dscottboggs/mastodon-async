use isolang::Language;

use mastodon_async_entities::visibility::Visibility;

/// Represents a post that can be sent to the POST /api/v1/status endpoint
///
/// See also [the API documentation](https://docs.joinmastodon.org/methods/statuses/#form-data-parameters)
#[derive(Debug, Default, Clone, Serialize, PartialEq, Eq)]
pub struct NewStatus {
    /// The text content of the status. If media_ids is provided, this becomes
    /// optional. Attaching a poll is optional while status is provided.
    ///
    /// Note that this means there is at this time no check provided by this
    /// type to ensure that this value is set when it is required by the API,
    /// and an APIError should be expected from [`crate::Mastodon::new_status()`]
    /// in this case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// ID of the status being replied to, if status is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to_id: Option<String>,
    /// Include Attachment IDs to be attached as media. If provided, status
    /// becomes optional, and poll cannot be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_ids: Option<Vec<String>>,
    /// Mark status and attached media as sensitive? Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Text to be shown as a warning or subject before the actual content.
    /// Statuses are generally collapsed behind this field.
    pub spoiler_text: Option<String>,
    /// Sets the visibility of the posted status to public, unlisted, private, direct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    /// ISO 639 language code for this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    /// Ignored by Mastodon servers, sets the content type for the status.
    /// Mastodon "toots" are always `text/plain`, regardless of this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}
