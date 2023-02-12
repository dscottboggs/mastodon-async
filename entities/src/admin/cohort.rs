use is_variant::IsVariant;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};
use crate::conversion;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, IsVariant)]
#[serde(rename_all = "lowercase")]
pub enum CohortFrequency {
    Day,
    Month,
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
    #[serde(with = "conversion::string_to::i64")]
    pub value: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cohort_example() {
        let example = r#"{
        	"period": "2022-09-01T00:00:00+00:00",
        	"frequency": "month",
        	"data": [
        		{
        			"date": "2022-09-01T00:00:00+00:00",
        			"rate": 1.0,
        			"value": "2"
        		}
        	]
        }"#;
        let subject: Cohort = serde_json::from_str(example).unwrap();
        assert!(subject.frequency.is_month());
        let data = &subject.data[0];
        assert_eq!(data.rate, 1.0);
        assert_eq!(data.value, 2);
    }
}
