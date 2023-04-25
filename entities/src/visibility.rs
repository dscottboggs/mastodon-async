use derive_is_enum_variant::is_enum_variant;
use serde::Deserialize;
use serde::Serialize;

/// The visibility of a status.
#[derive(
    Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, is_enum_variant,
)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    /// A Direct message to a user
    Direct,
    /// Only available to followers
    Private,
    /// Not shown in public timelines
    Unlisted,
    /// Posted to public timelines
    Public,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Public
    }
}

impl std::str::FromStr for Visibility {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "direct" => Ok(Visibility::Direct),
            "private" => Ok(Visibility::Private),
            "unlisted" => Ok(Visibility::Unlisted),
            "public" => Ok(Visibility::Public),
            invalid => Err(crate::error::Error::VisibilityParsingError {
                invalid: invalid.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_from_str() {
        assert!(Visibility::from_str("invalid")
            .expect_err("parsed invalid?")
            .is_visibility_parsing_error());
    }
}
