use serde::{Deserialize, Serialize};

use crate::{DimensionDataKey, DimensionKey};

/// Represents qualitative data about the server.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_Dimension/)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dimension {
    /// The unique keystring for the requested dimension.
    pub key: DimensionKey,
    /// The data available for the requested dimension.
    pub data: Vec<Data>,
}

/// An entry of data on a particular dimension of server metrics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data {
    /// The unique keystring for this data item.
    pub key: DimensionDataKey,
    /// A human-readable key for this data item.
    pub human_key: String,
    /// The value for this data item.
    pub value: String,
    /// The units associated with this data item’s value, if applicable.
    pub unit: Option<String>,
    /// A human-readable formatted value for this data item.
    pub human_value: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_usage_example() {
        let example = r#"{
        	"key": "space_usage",
        	"data": [
        		{
        			"key": "postgresql",
        			"human_key": "PostgreSQL",
        			"value": "14924935",
        			"unit": "bytes",
        			"human_value": "14.2 MB"
        		},
        		{
        			"key": "redis",
        			"human_key": "Redis",
        			"value": "1972544",
        			"unit": "bytes",
        			"human_value": "1.88 MB"
        		},
        		{
        			"key": "media",
        			"human_key": "Media storage",
        			"value": "0",
        			"unit": "bytes",
        			"human_value": "0 Bytes"
        		}
        	]
        }"#;
        let subject: Dimension = serde_json::from_str(example).unwrap();
        assert_eq!(subject.key, DimensionKey::new("space_usage"));
        let data = &subject.data[0];
        assert_eq!(data.key, DimensionDataKey::new("postgresql"));
        assert_eq!(data.human_key, "PostgreSQL");
        assert_eq!(data.value, "14924935");
        assert_eq!(data.unit.as_ref().unwrap(), "bytes");
        assert_eq!(data.human_value.as_ref().unwrap(), "14.2 MB");
    }
}