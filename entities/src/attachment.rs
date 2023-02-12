//! Module containing everything related to media attachements.

use crate::AttachmentId;
use derive_is_enum_variant::is_enum_variant;
use serde::{Deserialize, Serialize};
use url::Url;

/// Represents a file or media attachment that can be added to a status.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/MediaAttachment/)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Attachment {
    /// ID of the attachment.
    pub id: AttachmentId,
    /// The type of the attachment.
    #[serde(rename = "type")]
    pub media_type: MediaType,
    /// The location of the original full-size attachment, if the media has
    /// been processed. Is `None` if the media is still processing on the
    /// server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    /// The location of a scaled-down preview of the attachment.
    pub preview_url: Url,
    /// The location of the full-size original attachment on the remote website.
    pub remote_url: Option<Url>,
    /// Shorter URL for the image, for insertion into text
    /// (only present on local images)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_url: Option<Url>,
    /// Meta information about the attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    /// Noop will be removed.
    pub description: Option<String>,
    /// A hash computed by [the BlurHash algorithm](https://github.com/woltapp/blurhash),
    /// for generating colorful preview thumbnails when media has not been
    /// downloaded yet.
    pub blurhash: Option<String>,
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

/// Metadata about some attachment.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Meta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_encode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_bitrate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_channels: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<SizeSpecificDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small: Option<SizeSpecificDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<FocalPoint>,
}

/// The type of media attachment.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/MediaAttachment/#type)
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, is_enum_variant)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    /// Audio track
    Audio,
    /// An image.
    Image,
    /// A video file.
    Video,
    /// Looping, soundless animation
    Gifv,
    /// Unknown format.
    Unknown,
}

/// Metadata about a video attachment
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SizeSpecificDetails {
    /// How many pixels wide the video or image is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// How many pixels tall the video is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The frame rate of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<String>,
    /// The duration of the video, in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// The bitrate of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// The aspect ratio of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect: Option<f64>,
    /// The size of the video, expressed like WIDTHxHEIGHT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

/// A point on an image which should remain in view when cropped for previews.
///
/// There is some more information [here](https://docs.joinmastodon.org/api/guidelines/#focal-points)
/// and [here](https://docs.joinmastodon.org/entities/MediaAttachment/#meta) in
/// the API documentation.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub struct FocalPoint {
    /// The point on the horizontal plane which should remain in focus.
    pub x: f64,
    /// The point on the vertical plane which should remain in focus.
    pub y: f64,
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
    pub url: Url,
    /// For remote images, the remote URL of the original image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<Url>,
    /// URL of the preview image.
    pub preview_url: Url,
    /// Shorter URL for the image, for insertion into text
    /// (only present on local images)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_url: Option<Url>,
    /// Meta information about the attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    /// Noop will be removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::serde_value_test;

    use super::*;

    serde_value_test!(test_deserialize_with_image(Attachment): r##"{
  "id": "22345792",
  "type": "image",
  "url": "https://files.mastodon.social/media_attachments/files/022/345/792/original/57859aede991da25.jpeg",
  "preview_url": "https://files.mastodon.social/media_attachments/files/022/345/792/small/57859aede991da25.jpeg",
  "remote_url": null,
  "text_url": "https://mastodon.social/media/2N4uvkuUtPVrkZGysms",
  "meta": {
    "original": {
      "width": 640,
      "height": 480,
      "size": "640x480",
      "aspect": 1.3333333333333333
    },
    "small": {
      "width": 461,
      "height": 346,
      "size": "461x346",
      "aspect": 1.3323699421965318
    },
    "focus": {
      "x": -0.27,
      "y": 0.51
    }
  },
  "description": "test media description",
  "blurhash": "UFBWY:8_0Jxv4mx]t8t64.%M-:IUWGWAt6M}"
}"##);

    serde_value_test!(test_deserialize_with_audio(Attachment): r##"{
  "id": "21165404",
  "type": "audio",
  "url": "https://files.mastodon.social/media_attachments/files/021/165/404/original/a31a4a46cd713cd2.mp3",
  "preview_url": "https://files.mastodon.social/media_attachments/files/021/165/404/small/a31a4a46cd713cd2.mp3",
  "remote_url": null,
  "text_url": "https://mastodon.social/media/5O4uILClVqBWx0NNgvo",
  "meta": {
    "length": "0:06:42.86",
    "duration": 402.86,
    "audio_encode": "mp3",
    "audio_bitrate": "44100 Hz",
    "audio_channels": "stereo",
    "original": {
      "duration": 402.860408,
      "bitrate": 166290
    }
  },
  "description": null,
  "blurhash": null
}"##);

    serde_value_test!(test_deserialize_with_video(Attachment): r##"{
  "id": "22546306",
  "type": "video",
  "url": "https://files.mastodon.social/media_attachments/files/022/546/306/original/dab9a597f68b9745.mp4",
  "preview_url": "https://files.mastodon.social/media_attachments/files/022/546/306/small/dab9a597f68b9745.png",
  "remote_url": null,
  "text_url": "https://mastodon.social/media/wWd1HJIBmH1MZGDfg50",
  "meta": {
    "length": "0:01:28.65",
    "duration": 88.65,
    "fps": 24,
    "size": "1280x720",
    "width": 1280,
    "height": 720,
    "aspect": 1.7777777777777777,
    "audio_encode": "aac (LC) (mp4a / 0x6134706D)",
    "audio_bitrate": "44100 Hz",
    "audio_channels": "stereo",
    "original": {
      "width": 1280,
      "height": 720,
      "frame_rate": "6159375/249269",
      "duration": 88.654,
      "bitrate": 862056
    },
    "small": {
      "width": 400,
      "height": 225,
      "size": "400x225",
      "aspect": 1.7777777777777777
    }
  },
  "description": null,
  "blurhash": "U58E0g8_0M.94T?bIr00?bD%NGoM?bD%oLt7"
}"##);

    serde_value_test!(test_deserialize_with_gifv(Attachment): r##"{
  "id": "21130559",
  "type": "gifv",
  "url": "https://files.mastodon.social/media_attachments/files/021/130/559/original/bc84838f77991326.mp4",
  "preview_url": "https://files.mastodon.social/media_attachments/files/021/130/559/small/bc84838f77991326.png",
  "remote_url": null,
  "text_url": "https://mastodon.social/media/2ICiasGyjezmi7cQYM8",
  "meta": {
    "length": "0:00:01.11",
    "duration": 1.11,
    "fps": 33,
    "size": "600x332",
    "width": 600,
    "height": 332,
    "aspect": 1.8072289156626506,
    "original": {
      "width": 600,
      "height": 332,
      "frame_rate": "100/3",
      "duration": 1.11,
      "bitrate": 1627639
    },
    "small": {
      "width": 400,
      "height": 221,
      "size": "400x221",
      "aspect": 1.8099547511312217
    }
  },
  "description": null,
  "blurhash": "URHT%Jm,2a1d%MRO%LozkrNH$*n*oMn$Rjt7"
}
"##);
}
