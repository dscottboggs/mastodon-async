//! Module containing everything related to media attachements.

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
