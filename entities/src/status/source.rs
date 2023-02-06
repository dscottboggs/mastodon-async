use serde::{Deserialize, Serialize};

use crate::StatusId;

/// Represents a status's source as plain text.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/StatusSource/)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Source {
    /// ID of the status in the database.
    pub id: StatusId,
    /// The plain text used to compose the status.
    pub text: String,
    /// The plain text used to compose the status’s subject or content warning.
    pub spoiler_text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let example = r#"{
          "id": "108942703571991143",
          "text": "this is a status that will be edited",
          "spoiler_text": ""
        }"#;
        let subject: Source = serde_json::from_str(example).expect("deserialize");
        assert_eq!(subject.id, StatusId::new("108942703571991143"));
        assert_eq!(subject.text, "this is a status that will be edited");
        assert!(subject.spoiler_text.is_empty());
    }
}
