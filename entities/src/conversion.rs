pub(crate) mod string_to_u64 {
    use serde::{
        de::{self, Visitor},
        Deserializer, Serializer,
    };

    pub(crate) fn serialize<S>(value: &u64, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_str(&value.to_string())
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringToIntVisitor;

        impl<'v> Visitor<'v> for StringToIntVisitor {
            type Value = u64;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "a string which can be parsed as an unsigned, 64-bit integer"
                )
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse()
                    .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(v), &self))
            }
        }

        deserializer.deserialize_str(StringToIntVisitor)
    }
}

pub(crate) mod maybe_empty_url {

    use serde::{
        de::{self, Visitor},
        Deserializer, Serializer,
    };
    use url::Url;

    pub(crate) fn serialize<S>(value: &Option<Url>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_str(value.as_ref().map(AsRef::as_ref).unwrap_or_default())
    }
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Url>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MEUVisitor;

        impl<'v> Visitor<'v> for MEUVisitor {
            type Value = Option<Url>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a URL string or the empty string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(v.parse().map_err(|_| {
                        de::Error::invalid_value(de::Unexpected::Str(v), &self)
                    })?))
                }
            }
        }

        deserializer.deserialize_str(MEUVisitor)
    }
}

pub mod date_from_timestamp {
    use serde::{de, de::Visitor, Deserializer, Serializer};
    use time::{Date, OffsetDateTime, PrimitiveDateTime, Time};

    pub(crate) fn serialize<S>(value: &Date, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let time = PrimitiveDateTime::new(
            *value,
            Time::from_hms(0, 0, 0).map_err(|err| serde::ser::Error::custom(format!("{err:?}")))?,
        )
        .assume_utc();
        ser.serialize_str(&time.unix_timestamp().to_string())
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Date, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Vizitor;

        impl<'v> Visitor<'v> for Vizitor {
            type Value = Date;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "a string containing an integer representing a unix timestamp"
                )
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let invalid = || de::Error::invalid_value(de::Unexpected::Str(v), &self);
                let n = v.parse().map_err(|_| invalid())?;
                let it = OffsetDateTime::from_unix_timestamp(n).map_err(|_| invalid())?;
                Ok(it.date())
            }
        }
        deserializer.deserialize_str(Vizitor)
    }
}
