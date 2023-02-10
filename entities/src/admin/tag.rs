use serde::{Deserialize, Serialize};

use crate::{tag, TagId};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    /// Non-admin data related to this hashtag
    #[serde(flatten)]
    pub tag: tag::Tag,
    /// The ID of the Tag in the database.
    pub id: TagId,
    /// Whether the hashtag has been approved to trend.
    pub trendable: bool,
    /// Whether the hashtag has not been disabled from auto-linking.
    pub usable: bool,
    /// Whether the hashtag has not been reviewed yet to approve or deny its
    /// trending.
    pub requires_review: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let example = r#"{
          "name": "caturday",
          "url": "https://mastodon.example/tags/caturday",
          "history": [
            {
              "day": "1669507200",
              "accounts": "53",
              "uses": "56"
            },
            {
              "day": "1669420800",
              "accounts": "142",
              "uses": "171"
            },
            {
              "day": "1669334400",
              "accounts": "11",
              "uses": "11"
            },
            {
              "day": "1669248000",
              "accounts": "8",
              "uses": "9"
            },
            {
              "day": "1669161600",
              "accounts": "8",
              "uses": "20"
            },
            {
              "day": "1669075200",
              "accounts": "11",
              "uses": "11"
            },
            {
              "day": "1668988800",
              "accounts": "17",
              "uses": "22"
            }
          ],
          "id": "802",
          "trendable": true,
          "usable": true,
          "requires_review": false
        }"#;
        let tag: Tag = serde_json::from_str(example).expect("deserialize");
        assert_eq!(tag.tag.name, "caturday");
        assert_eq!(tag.id, TagId::new("802"));
        assert!(tag.trendable);
        assert!(tag.usable);
        assert!(!tag.requires_review);
    }
}
