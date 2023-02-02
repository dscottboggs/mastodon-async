use is_variant::IsVariant;
use isolang::Language;
use serde::{Deserialize, Serialize};

use crate::prelude::Visibility;

/// Represents a user's preferences.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Preferences/)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "PreferencesSerializer", into = "PreferencesSerializer")]
pub struct Preferences {
    /// Preferences related to posts
    pub posting: PostingPreferences,
    /// How a user prefers to read their feed
    pub reading: ReadingPreferences,
}

/// User post preferences
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostingPreferences {
    /// Defaults for new posts
    pub default: PostDefaults,
}

/// Defaults for new posts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostDefaults {
    /// Default visibility for new posts
    pub visibility: Visibility,
    /// Default sensitivity flag for new posts
    pub sensitive: bool,
    /// Default language for new posts.
    pub language: Option<Language>,
}

/// How a user prefers to read their feed
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ReadingPreferences {
    /// Whether certain elements of the feed should be expanded/made visible or not.
    pub expand: ReadingExpansionPreferences,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ReadingExpansionPreferences {
    /// Whether media attachments should be automatically displayed or blurred/hidden.
    pub media: MediaExpansion,
    /// Whether CWs should be expanded by default.
    pub spoilers: bool,
}

impl From<Preferences> for PreferencesSerializer {
    fn from(value: Preferences) -> Self {
        PreferencesSerializer {
            posting_default_visibility: value.posting.default.visibility,
            posting_default_sensitive: value.posting.default.sensitive,
            posting_default_language: value.posting.default.language,
            reading_expand_media: value.reading.expand.media,
            reading_expand_spoilers: value.reading.expand.spoilers,
        }
    }
}

impl From<PreferencesSerializer> for Preferences {
    fn from(value: PreferencesSerializer) -> Self {
        Self {
            posting: PostingPreferences {
                default: PostDefaults {
                    visibility: value.posting_default_visibility,
                    sensitive: value.posting_default_sensitive,
                    language: value.posting_default_language,
                },
            },
            reading: ReadingPreferences {
                expand: ReadingExpansionPreferences {
                    media: value.reading_expand_media,
                    spoilers: value.reading_expand_spoilers,
                },
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
struct PreferencesSerializer {
    /// Default visibility for new posts
    #[serde(rename = "posting:default:visibility")]
    posting_default_visibility: Visibility,
    /// Default sensitivity flag for new posts
    #[serde(rename = "posting:default:sensitive")]
    posting_default_sensitive: bool,
    /// Default language for new posts.
    #[serde(rename = "posting:default:language")]
    posting_default_language: Option<Language>,
    /// Whether media attachments should be automatically displayed or blurred/hidden.
    #[serde(rename = "reading:expand:media")]
    reading_expand_media: MediaExpansion,
    /// Whether CWs should be expanded by default.
    #[serde(rename = "reading:expand:spoilers")]
    reading_expand_spoilers: bool,
}

/// Whether media attachments should be automatically displayed or blurred/hidden.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum MediaExpansion {
    /// Hide media marked as sensitive
    Default,
    /// Always show all media by default, regardless of sensitivity
    ShowAll,
    /// Always hide all media by default, regardless of sensitivity
    HideAll,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let example = r#"{
            "posting:default:visibility": "public",
            "posting:default:sensitive": false,
            "posting:default:language": null,
            "reading:expand:media": "default",
            "reading:expand:spoilers": false
        }"#;
        let subject: Preferences = serde_json::from_str(example).expect("deserialize");
        assert!(subject.posting.default.visibility.is_public());
        assert!(!subject.posting.default.sensitive);
        assert!(subject.posting.default.language.is_none());
        assert!(subject.reading.expand.media.is_default());
    }
}
