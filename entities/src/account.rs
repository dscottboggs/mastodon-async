//! A module containing everything relating to a account returned from the api.

use serde::{
    de::{self, Deserializer, Unexpected},
    Deserialize, Serialize,
};
use std::path::PathBuf;
use time::{serde::iso8601, OffsetDateTime};

/// A struct representing an Account.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Account {
    /// Equals `username` for local users, includes `@domain` for remote ones.
    pub acct: String,
    /// URL to the avatar image
    pub avatar: String,
    /// URL to the avatar static image (gif)
    pub avatar_static: String,
    /// The time the account was created.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// The account's display name.
    pub display_name: String,
    /// The number of followers for the account.
    pub followers_count: u64,
    /// The number of accounts the given account is following.
    pub following_count: u64,
    /// URL to the header image.
    pub header: String,
    /// URL to the header static image (gif).
    pub header_static: String,
    /// The ID of the account.
    pub id: AccountId,
    /// Boolean for when the account cannot be followed without waiting for
    /// approval first.
    pub locked: bool,
    /// Biography of user.
    pub note: String,
    /// The number of statuses the account has made.
    pub statuses_count: u64,
    /// URL of the user's profile page (can be remote).
    pub url: String,
    /// The username of the account.
    pub username: String,
    /// An extra attribute given from `verify_credentials` giving defaults about
    /// a user
    pub source: Option<Source>,
    /// If the owner decided to switch accounts, new account is in
    /// this attribute
    pub moved: Option<Box<Account>>,
    /// List of profile metadata fields
    pub fields: Option<Vec<MetadataField>>,
    /// Boolean indicating whether this account is a bot or not
    pub bot: Option<bool>,
}

/// Wrapper type for a account ID string
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct AccountId(String);

impl AsRef<str> for AccountId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AccountId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

/// A single name: value pair from a user's profile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MetadataField {
    /// name part of metadata
    pub name: String,
    /// value part of metadata
    pub value: String,
}

impl MetadataField {
    pub fn new(name: &str, value: &str) -> MetadataField {
        MetadataField {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// An extra object given from `verify_credentials` giving defaults about a user
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Source {
    privacy: Option<crate::visibility::Visibility>,
    #[serde(deserialize_with = "string_or_bool")]
    sensitive: bool,
    note: Option<String>,
    fields: Option<Vec<MetadataField>>,
}

fn string_or_bool<'de, D: Deserializer<'de>>(val: D) -> ::std::result::Result<bool, D::Error> {
    #[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
    #[serde(untagged)]
    pub enum BoolOrString {
        Bool(bool),
        Str(String),
    }

    Ok(match BoolOrString::deserialize(val)? {
        BoolOrString::Bool(b) => b,
        BoolOrString::Str(ref s) => {
            if s == "true" {
                true
            } else if s == "false" {
                false
            } else {
                return Err(de::Error::invalid_value(
                    Unexpected::Str(s),
                    &"true or false",
                ));
            }
        }
    })
}

#[derive(Debug, Default, Clone, Serialize, PartialEq, Eq)]
pub struct UpdateSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<crate::visibility::Visibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
}

#[derive(Debug, Default, Serialize, PartialEq, Eq)]
pub struct Credentials {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<UpdateSource>,
    #[serde(serialize_with = "fields_attributes_ser::ser")]
    pub fields_attributes: Vec<MetadataField>,
}

mod fields_attributes_ser {
    use super::*;
    use serde::ser::{SerializeMap, Serializer};
    pub fn ser<S>(attrs: &Vec<MetadataField>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(attrs.len()))?;
        for (i, field) in attrs.iter().enumerate() {
            map.serialize_entry(&i, &field)?;
        }
        map.end()
    }
}
