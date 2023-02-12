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
    #[serde(with = "conversion::string_to::u64")]
    pub total: u64,
    /// A human-readable formatted value for this data item.
    #[serde(default)]
    pub human_value: Option<String>,
    /// The numeric total associated with the requested measure, in the previous
    /// period. Previous period is calculated by subtracting the start_at and
    /// end_at dates, then offsetting both start and end dates backwards by the
    /// length of the time period.
    #[serde(with = "conversion::string_to::u64::option")]
    pub previous_total: Option<u64>,
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
    #[serde(with = "conversion::string_to::u64")]
    pub value: u64,
}

#[cfg(test)]
mod tests {
    use time::Month;

    use super::*;

    #[test]
    fn test_measure_example() {
        let example = r#"{
          "key": "active_users",
          "unit": null,
          "total": "2",
          "previous_total": "0",
          "data": [
            {
              "date": "2022-09-14T00:00:00Z",
              "value": "0"
            },
            {
              "date": "2022-09-15T00:00:00Z",
              "value": "0"
            },
            {
              "date": "2022-09-16T00:00:00Z",
              "value": "0"
            },
            {
              "date": "2022-09-17T00:00:00Z",
              "value": "1"
            },
            {
              "date": "2022-09-18T00:00:00Z",
              "value": "1"
            },
            {
              "date": "2022-09-19T00:00:00Z",
              "value": "1"
            },
            {
              "date": "2022-09-20T00:00:00Z",
              "value": "2"
            },
            {
              "date": "2022-09-21T00:00:00Z",
              "value": "1"
            }
          ]
        }"#;
        let subject: Measure = serde_json::from_str(example).unwrap();
        assert_eq!(subject.key, MeasureKey::new("active_users"));
        assert!(subject.unit.is_none());
        assert_eq!(subject.total, 2);
        assert_eq!(subject.previous_total, Some(0));
        assert!(subject.human_value.is_none());
        let data = &subject.data[0];
        assert_eq!(data.value, 0);
        let date = data.date.date();
        assert_eq!(date.year(), 2022);
        assert_eq!(date.month(), Month::September);
        assert_eq!(date.day(), 14);
    }
}