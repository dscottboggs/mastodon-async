//! Module representing cards of statuses.

use crate::{conversion, tag};
use is_variant::IsVariant;
use serde::{Deserialize, Serialize};
use url::Url;

/// Represents a rich preview card that is generated using OpenGraph tags from a URL.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/PreviewCard/)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Card {
    /// Location of linked resource.
    pub url: Url,
    /// Title of linked resource.
    pub title: String,
    /// Description of preview.
    pub description: String,
    /// The type of the preview card.
    #[serde(rename = "type")]
    pub card_type: CardType,
    /// Preview thumbnail.
    pub image: Option<String>,
    /// The author of the original resource.
    pub author_name: String,
    /// A link to the author of the original resource.
    #[serde(with = "conversion::maybe_empty_url")]
    pub author_url: Option<Url>,
    /// The provider of the original resource.
    pub provider_name: String,
    /// A link to the provider of the original resource.
    #[serde(with = "conversion::maybe_empty_url")]
    pub provider_url: Option<Url>,
    /// HTML to be used for generating the preview card.
    pub html: String,
    /// Width of preview, in pixels. When [`card_type`](Card::card_type) is `Link`, this is `0`.
    pub width: u64,
    /// Height of preview, in pixels. When [`card_type`](Card::card_type) is `Link`, this is `0`.
    pub height: u64,
    /// Used for photo embeds, instead of custom html.
    #[serde(with = "conversion::maybe_empty_url")]
    pub embed_url: Option<Url>,
    /// A hash computed by [the BlurHash algorithm](https://github.com/woltapp/blurhash),
    /// for generating colorful preview thumbnails when media has not been
    /// downloaded yet.
    pub blurhash: Option<String>,
}

/// The type of the preview card.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, IsVariant)]
#[serde(rename_all = "lowercase")]
pub enum CardType {
    /// Link OEmbed
    Link,
    /// Photo OEmbed
    Photo,
    /// Video OEmbed
    Video,
    /// iframe OEmbed. Not currently accepted, so won’t show up in practice.
    Rich,
}

/// A preview card which holds a trending link
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct TrendsLink {
    /// The preview card associated with this trending link
    #[serde(flatten)]
    pub card: Card,
    /// The history of this trend
    pub history: Vec<tag::History>,
}

#[cfg(test)]
mod tests {
    use crate::serde_value_test;

    use super::*;

    serde_value_test!(test_video(Card): r#"{
        "url": "https://www.youtube.com/watch?v=OMv_EPMED8Y",
        "title": "♪ Brand New Friend (Christmas Song!)",
        "description": "",
        "type": "video",
        "author_name": "YOGSCAST Lewis & Simon",
        "author_url": "https://www.youtube.com/user/BlueXephos",
        "provider_name": "YouTube",
        "provider_url": "https://www.youtube.com/",
        "html": "<iframe width=\"480\" height=\"270\" src=\"https://www.youtube.com/embed/OMv_EPMED8Y?feature=oembed\" frameborder=\"0\" allowfullscreen=\"\"></iframe>",
        "width": 480,
        "height": 270,
        "image": "https://files.mastodon.social/preview_cards/images/014/179/145/original/9cf4b7cf5567b569.jpeg",
        "embed_url": "",
        "blurhash": "UvK0HNkV,:s9xBR%njog0fo2W=WBS5ozofV@"
    }"#);

    serde_value_test!(test_photo(Card): r#"{
        "url": "https://www.flickr.com/photos/tomfenskephotography/49088768431/",
        "title": "Oregon",
        "description": "",
        "type": "photo",
        "author_name": "Tom Fenske Photography",
        "author_url": "https://www.flickr.com/photos/tomfenskephotography/",
        "provider_name": "Flickr",
        "provider_url": "https://www.flickr.com/",
        "html": "",
        "width": 1024,
        "height": 427,
        "image": "https://files.mastodon.social/preview_cards/images/014/287/139/original/651b1c6976817824.jpeg",
        "embed_url": "https://live.staticflickr.com/65535/49088768431_6a4322b3bb_b.jpg",
        "blurhash": "UnE{@jt6M_oIAhjYs+ayT2WBf9ayRkkDXAj["
    }"#);

    serde_value_test!(test_link(Card): r#"{
        "url": "https://www.theguardian.com/money/2019/dec/07/i-lost-my-193000-inheritance-with-one-wrong-digit-on-my-sort-code",
        "title": "‘I lost my £193,000 inheritance – with one wrong digit on my sort code’",
        "description": "When Peter Teich’s money went to another Barclays customer, the bank offered £25 as a token gesture",
        "type": "link",
        "author_name": "",
        "author_url": "",
        "provider_name": "",
        "provider_url": "",
        "html": "",
        "width": 0,
        "height": 0,
        "image": null,
        "embed_url": "",
        "blurhash": null
    }"#);

    serde_value_test!(test_trending_link(TrendsLink): r#"{
        "url": "https://www.nbcnews.com/specials/plan-your-vote-2022-elections/index.html",
        "title": "Plan Your Vote: 2022 Elections",
        "description": "Everything you need to know about the voting rules where you live, including registration, mail-in voting, changes since 2020, and more.",
        "type": "link",
        "author_name": "NBC News",
        "author_url": "",
        "provider_name": "NBC News",
        "provider_url": "",
        "html": "",
        "width": 400,
        "height": 225,
        "image": "https://files.mastodon.social/cache/preview_cards/images/045/027/478/original/0783d5e91a14fd49.jpeg",
        "embed_url": "",
        "blurhash": "UcQmF#ay~qofj[WBj[j[~qof9Fayofofayay",
        "history": [
          {
            "day": "1661817600",
            "accounts": "7",
            "uses": "7"
          },
          {
            "day": "1661731200",
            "accounts": "23",
            "uses": "23"
          },
          {
            "day": "1661644800",
            "accounts": "0",
            "uses": "0"
          },
          {
            "day": "1661558400",
            "accounts": "0",
            "uses": "0"
          },
          {
            "day": "1661472000",
            "accounts": "0",
            "uses": "0"
          },
          {
            "day": "1661385600",
            "accounts": "0",
            "uses": "0"
          },
          {
            "day": "1661299200",
            "accounts": "0",
            "uses": "0"
          }
        ]
    }"#);
}
