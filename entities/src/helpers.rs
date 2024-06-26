/// Returns true if the given value refers to "false"
pub fn is_false(value: &bool) -> bool {
    !*value
}

pub(crate) mod serde_opt_duration_as_seconds {
    use time::{ext::NumericalDuration, Duration};

    use serde::de;

    pub(crate) fn serialize<S>(
        duration: &Option<Duration>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(duration) = duration {
            serializer.serialize_i64(duration.whole_seconds())
        } else {
            serializer.serialize_none()
        }
    }

    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<Duration>, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Visitor;

        struct DurationVisitor;

        impl<'v> Visitor<'v> for DurationVisitor {
            type Value = Option<Duration>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "signed 64-bit integer")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Some(v.seconds()))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                i64::try_from(v)
                    .map(|v| Some(v.seconds()))
                    .map_err(|_| de::Error::invalid_value(de::Unexpected::Unsigned(v), &self))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.is_empty() {
                    Ok(None)
                } else {
                    v.parse()
                        .map(|n: i64| Some(n.seconds()))
                        .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(v), &self))
                }
            }
            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }
        }
        deserializer.deserialize_any(DurationVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use time::{ext::NumericalDuration, Duration};

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct TestDuration {
        #[serde(
            with = "serde_opt_duration_as_seconds",
            skip_serializing_if = "Option::is_none",
            default
        )]
        dur: Option<Duration>,
    }

    impl Default for TestDuration {
        fn default() -> Self {
            TestDuration {
                dur: Some(10.seconds()),
            }
        }
    }

    impl TestDuration {
        fn empty() -> Self {
            Self { dur: None }
        }
    }

    #[test]
    fn test_serialize_duration() {
        let it = TestDuration::default();
        let serialized = serde_json::to_string(&it).expect("serialize");
        assert_eq!(serialized, r#"{"dur":10}"#);
    }

    #[test]
    fn test_serialize_empty_duration() {
        let it = TestDuration::empty();
        let ser = serde_json::to_string(&it).expect("serialize");
        assert_eq!("{}", ser);
    }

    #[test]
    fn test_deserialize_duration() {
        let text = r#"{"dur": 10}"#;
        let duration: TestDuration = serde_json::from_str(text).expect("deserialize");
        assert_eq!(duration.dur.unwrap().whole_seconds(), 10);
        let text = r#"{"dur": "10"}"#;
        let duration: TestDuration = serde_json::from_str(text).expect("deserialize");
        assert_eq!(duration.dur.unwrap().whole_seconds(), 10);
    }

    #[test]
    fn test_deserialize_empty_duration() {
        let text = r#"{"dur": ""}"#;
        let duration: TestDuration = serde_json::from_str(text).expect("deserialize");
        assert!(duration.dur.is_none());
    }

    #[test]
    fn test_deserialize_null_duration() {
        let text = r#"{}"#;
        let duration: TestDuration = serde_json::from_str(text).expect("deserialize");
        assert!(duration.dur.is_none());
    }
}
