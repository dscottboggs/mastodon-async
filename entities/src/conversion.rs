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
