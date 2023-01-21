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
