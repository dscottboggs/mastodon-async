use serde::{Deserialize, Serialize};

use crate::{DimensionDataKey, DimensionKey};

/// Represents qualitative data about the server.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_Dimension/)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    /// The unique keystring for the requested dimension.
    pub key: DimensionKey,
    /// The data available for the requested dimension.
    pub data: Vec<Data>,
}

/// An entry of data on a particular dimension of server metrics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
