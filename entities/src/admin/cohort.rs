use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// Represents a retention metric.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_Cohort/)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cohort {
    /// The timestamp for the start of the period, at midnight.
    #[serde(with = "iso8601")]
    pub period: OffsetDateTime,
    /// The size of the bucket for the returned data.
    pub frequency: CohortFrequency,
    /// Retention data for users who registered during the given period.
    pub data: Vec<Data>,
}

/// The size of the bucket for the returned [`Cohort`] data.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_Cohort/#frequency)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CohortFrequency {
    Day,
    Month,
}

impl CohortFrequency {
    #![allow(missing_docs)]

    pub fn is_day(&self) -> bool {
        *self == Self::Day
    }
    pub fn is_month(&self) -> bool {
        *self == Self::Month
    }
}

/// Represents a single value from a set of retention metrics.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_Cohort/#CohortData)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    /// The timestamp for the start of the bucket, at midnight.
    #[serde(with = "iso8601")]
    pub date: OffsetDateTime,
    /// The percentage rate of users who registered in the specified period and
    /// were active for the given date bucket.
    pub rate: f64,
    /// How many users registered in the specified period and were active for the given date bucket.
    pub value: i64,
}
