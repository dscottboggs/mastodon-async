use serde::Deserialize;
use serde::Serialize;

/// The visibility of a status.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
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
