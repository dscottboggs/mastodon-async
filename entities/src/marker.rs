use crate::StatusId;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// Represents the last read position within a user's timelines.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Marker/)
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Marker {
    /// The ID of the most recently viewed entity.
    pub last_read_id: StatusId,
    /// An incrementing counter, used for locking to prevent write conflicts.
    pub version: i64,
    #[serde(with = "iso8601")]
    pub updated_at: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use time::format_description::well_known::Iso8601;

    use super::*;

    #[test]
    fn test_deserialize() {
        let example = r#"{
          "last_read_id": "103194548672408537",
          "version": 462,
          "updated_at": "2019-11-24T19:39:39.337Z"
        }"#;
        let subject: Marker = serde_json::from_str(example).expect("deserialize");
        assert_eq!(subject.last_read_id, StatusId::new("103194548672408537"));
        assert_eq!(subject.version, 462);
        assert_eq!(
            subject.updated_at,
            OffsetDateTime::parse("2019-11-24T19:39:39.337Z", &Iso8601::PARSING)
                .expect("parse updated time")
        );
    }
}
