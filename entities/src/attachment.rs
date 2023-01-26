//! Module containing everything related to media attachements.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// A struct representing a media attachment.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Attachment {
    /// ID of the attachment.
    pub id: AttachmentId,
    /// The media type of an attachment.
    #[serde(rename = "type")]
    pub media_type: MediaType,
    /// URL of the locally hosted version of the image.
    pub url: Option<String>,
    /// For remote images, the remote URL of the original image.
    pub remote_url: Option<String>,
    /// URL of the preview image.
    pub preview_url: String,
    /// Shorter URL for the image, for insertion into text
    /// (only present on local images)
    pub text_url: Option<String>,
    /// Meta information about the attachment.
    pub meta: Option<Meta>,
    /// Noop will be removed.
    pub description: Option<String>,
}

impl Attachment {
    /// If this is an attachment which was either processed synchronously or
    /// in some other way has finished processing before being deserialized,
    /// `url` will be present. This is a convenience method to indicate that
    /// state.
    ///
    /// If possible, it's recommended instead to use
    /// [`Mastodon::wait_for_processing()`](https://docs.rs/mastodon-async/latest/mastodon_async/mastodon/struct.Mastodon.html#method.wait_for_processing).
    pub fn is_done_processing(&self) -> bool {
        self.url.is_some()
    }
}
/// Wrapper type for a attachment ID string
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct AttachmentId(String);

impl AsRef<str> for AttachmentId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AttachmentId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl Display for AttachmentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

static_assertions::assert_not_impl_any!(
    AttachmentId: PartialEq<crate::account::AccountId>,
    PartialEq<crate::filter::FilterId>,
    PartialEq<crate::list::ListId>,
    PartialEq<crate::mention::MentionId>,
    PartialEq<crate::notification::NotificationId>,
    PartialEq<crate::relationship::RelationshipId>,
    PartialEq<crate::report::ReportId>,
    PartialEq<crate::push::SubscriptionId>,
    PartialEq<crate::status::StatusId>,
);

/// Information about the attachment itself.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Meta {
    /// Original version.
    pub original: Option<ImageDetails>,
    /// Smaller version.
    pub small: Option<ImageDetails>,
}

/// Dimensions of an attachement.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ImageDetails {
    /// width of attachment.
    pub width: u64,
    /// height of attachment.
    pub height: u64,
    /// A string of `widthxheight`.
    pub size: Option<String>,
    /// The aspect ratio of the attachment.
    pub aspect: Option<f64>,
}

/// The type of media attachment.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    /// An image.
    #[serde(rename = "image")]
    Image,
    /// A video file.
    #[serde(rename = "video")]
    Video,
    /// A gifv format file.
    #[serde(rename = "gifv")]
    Gifv,
    /// Unknown format.
    #[serde(rename = "unknown")]
    Unknown,
}

/// A media attachment which has been processed and has a URL.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProcessedAttachment {
    /// ID of the attachment.
    pub id: AttachmentId,
    /// The media type of an attachment.
    #[serde(rename = "type")]
    pub media_type: MediaType,
    /// URL of the locally hosted version of the image.
    pub url: String,
    /// For remote images, the remote URL of the original image.
    pub remote_url: Option<String>,
    /// URL of the preview image.
    pub preview_url: String,
    /// Shorter URL for the image, for insertion into text
    /// (only present on local images)
    pub text_url: Option<String>,
    /// Meta information about the attachment.
    pub meta: Option<Meta>,
    /// Noop will be removed.
    pub description: Option<String>,
}
