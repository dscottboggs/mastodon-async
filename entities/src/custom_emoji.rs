use serde::{Deserialize, Serialize};
use url::Url;

/// Represents a custom emoji
///
/// See <https://docs.joinmastodon.org/entities/CustomEmoji/>
///
/// ## Example
/// ```rust
/// use mastodon_async_entities::custom_emoji::CustomEmoji;
/// let example_serialized = r#"{
///   "shortcode": "blobaww",
///   "url": "https://files.mastodon.social/custom_emojis/images/000/011/739/original/blobaww.png",
///   "static_url": "https://files.mastodon.social/custom_emojis/images/000/011/739/static/blobaww.png",
///   "visible_in_picker": true,
///   "category": "Blobs"
/// }"#;
/// let emoji: CustomEmoji = serde_json::from_str(example_serialized).unwrap();
/// assert_eq!(emoji.shortcode, "blobaww");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct CustomEmoji {
    /// The name of the custom emoji.
    ///
    /// See <https://docs.joinmastodon.org/entities/CustomEmoji/#shortcode>
    pub shortcode: String,
    /// A link to the custom emoji.
    ///
    /// See <https://docs.joinmastodon.org/entities/CustomEmoji/#url>
    pub url: Url,
    /// A link to a static copy of the custom emoji. Only different from [`url`]
    /// when the emoji is animated.
    ///
    /// See <https://docs.joinmastodon.org/entities/CustomEmoji/#static_url>
    pub static_url: Url,
    /// Whether this Emoji should be visible in the picker or unlisted.
    ///
    /// <https://docs.joinmastodon.org/entities/CustomEmoji/#visible_in_picker>
    pub visible_in_picker: bool,
    /// Used for sorting custom emoji in the picker.
    ///
    /// See <https://docs.joinmastodon.org/entities/CustomEmoji/#category>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}
