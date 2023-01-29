use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

use crate::{conversion, MeasureKey};

/// Represents quantitative data about the server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Measure {
    /// The unique keystring for the requested measure.
    pub key: MeasureKey,
    /// The units associated with this data item’s value, if applicable.
    pub unit: Option<String>,
    /// The numeric total associated with the requested measure.
    #[serde(with = "conversion::string_to_u64")]
    pub total: u64,
    /// A human-readable formatted value for this data item.
    pub human_value: String,
    /// The numeric total associated with the requested measure, in the previous
    /// period. Previous period is calculated by subtracting the start_at and
    /// end_at dates, then offsetting both start and end dates backwards by the
    /// length of the time period.
    #[serde(with = "conversion::string_to_u64")]
    pub previous_total: u64,
    /// The data available for the requested measure, split into daily buckets.
    pub data: Vec<Data>,
}

/// One day's bucket of data in a measure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data {
    /// Midnight on the requested day in the time period.
    #[serde(with = "iso8601")]
    pub date: OffsetDateTime,
    /// The numeric value for the requested measure.
    #[serde(with = "conversion::string_to_u64")]
    pub value: u64,
}
