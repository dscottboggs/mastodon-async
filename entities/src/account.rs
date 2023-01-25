//! A module containing everything relating to a account returned from the api.

use derive_builder::Builder;
use isolang::Language;
use serde::{
    de::{self, Deserializer, Unexpected, Visitor},
    Deserialize, Serialize,
};
use std::{num::ParseIntError, path::PathBuf, str::FromStr};
use time::{serde::iso8601, OffsetDateTime};
use url::Url;

use crate::custom_emoji::CustomEmoji;

use crate::AccountId;

/// A struct representing an Account.
///
/// See also [the API reference](https://docs.joinmastodon.org/entities/Account/)
///
/// ## Example
/// ```rust
/// use mastodon_async_entities::account::Account;
/// let example = r##"{
///   "id": "23634",
///   "username": "noiob",
///   "acct": "noiob@awoo.space",
///   "display_name": "ikea shark fan account",
///   "locked": false,
///   "bot": false,
///   "created_at": "2017-02-08T02:00:53.274Z",
///   "note": "<p>:ms_rainbow_flag:​ :ms_bisexual_flagweb:​ :ms_nonbinary_flag:​ <a href=\"https://awoo.space/tags/awoo\" class=\"mention hashtag\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">#<span>awoo</span}.space <a href=\"https://awoo.space/tags/admin\" class=\"mention hashtag\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">#<span>admin</span} ~ <a href=\"https://awoo.space/tags/bi\" class=\"mention hashtag\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">#<span>bi</span} ~ <a href=\"https://awoo.space/tags/nonbinary\" class=\"mention hashtag\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">#<span>nonbinary</span} ~ compsci student ~ likes video <a href=\"https://awoo.space/tags/games\" class=\"mention hashtag\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">#<span>games</span} and weird/ old electronics and will post obsessively about both ~ avatar by <span class=\"h-card\"><a href=\"https://weirder.earth/@dzuk\" class=\"u-url mention\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">@<span>dzuk</span}</span></p>",
///   "url": "https://awoo.space/@noiob",
///   "avatar": "https://files.mastodon.social/accounts/avatars/000/023/634/original/6ca8804dc46800ad.png",
///   "avatar_static": "https://files.mastodon.social/accounts/avatars/000/023/634/original/6ca8804dc46800ad.png",
///   "header": "https://files.mastodon.social/accounts/headers/000/023/634/original/256eb8d7ac40f49a.png",
///   "header_static": "https://files.mastodon.social/accounts/headers/000/023/634/original/256eb8d7ac40f49a.png",
///   "followers_count": 547,
///   "following_count": 404,
///   "statuses_count": 28468,
///   "last_status_at": "2019-11-17T00:02:23.693Z",
///   "emojis": [
///     {
///       "shortcode": "ms_rainbow_flag",
///       "url": "https://files.mastodon.social/custom_emojis/images/000/028/691/original/6de008d6281f4f59.png",
///       "static_url": "https://files.mastodon.social/custom_emojis/images/000/028/691/static/6de008d6281f4f59.png",
///       "visible_in_picker": true
///     },
///     {
///       "shortcode": "ms_bisexual_flag",
///       "url": "https://files.mastodon.social/custom_emojis/images/000/050/744/original/02f94a5fca7eaf78.png",
///       "static_url": "https://files.mastodon.social/custom_emojis/images/000/050/744/static/02f94a5fca7eaf78.png",
///       "visible_in_picker": true
///     },
///     {
///       "shortcode": "ms_nonbinary_flag",
///       "url": "https://files.mastodon.social/custom_emojis/images/000/105/099/original/8106088bd4782072.png",
///       "static_url": "https://files.mastodon.social/custom_emojis/images/000/105/099/static/8106088bd4782072.png",
///       "visible_in_picker": true
///     }
///   ],
///   "fields": [
///     {
///       "name": "Pronouns",
///       "value": "they/them",
///       "verified_at": null
///     },
///     {
///       "name": "Alt",
///       "value": "<span class=\"h-card\"><a href=\"https://cybre.space/@noiob\" class=\"u-url mention\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">@<span>noiob</span}</span>",
///       "verified_at": null
///     },
///     {
///       "name": "Bots",
///       "value": "<span class=\"h-card\"><a href=\"https://botsin.space/@darksouls\" class=\"u-url mention\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">@<span>darksouls</span}</span>, <span class=\"h-card\"><a href=\"https://botsin.space/@nierautomata\" class=\"u-url mention\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">@<span>nierautomata</span}</span>, <span class=\"h-card\"><a href=\"https://mastodon.social/@fedi\" class=\"u-url mention\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">@<span>fedi</span}</span>, code for <span class=\"h-card\"><a href=\"https://botsin.space/@awoobot\" class=\"u-url mention\" rel=\"nofollow noopener noreferrer\" target=\"_blank\">@<span>awoobot</span}</span>",
///       "verified_at": null
///     },
///     {
///       "name": "Website",
///       "value": "<a href=\"http://shork.xyz\" rel=\"nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">http://</span><span class=\"\">shork.xyz</span><span class=\"invisible\"></span}",
///       "verified_at": "2019-11-10T10:31:10.744+00:00"
///     }
///   ]
/// }"##;
/// let subject: Account = serde_json::from_str(example).unwrap();
/// assert_eq!(subject.username, "noiob")
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Account {
    /// The Webfinger account URI. Equal to [`Account::username`] for local users, or
    /// `username@domain` for remote users. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#acct)
    pub acct: String,
    /// An image icon that is shown next to statuses and in the profile. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#avatar)
    pub avatar: Url,
    /// A static version of the avatar. Equal to avatar if its value is a static
    /// image; different if avatar is an animated GIF. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#avatar_static)
    pub avatar_static: Url,
    /// Indicates that the account may perform automated actions, may not be
    /// monitored, or identifies as a robot. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#bot)
    pub bot: Option<bool>,
    /// The time the account was created. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#created_at)
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// Whether the account has opted into discovery features such as the
    /// profile directory. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#discoverable)
    pub discoverable: Option<bool>,
    /// The account's display name. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#display_name)
    pub display_name: String,
    /// Custom emoji entities to be used when rendering the profile. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#emojis)
    #[serde(default = "Vec::new")]
    pub emojis: Vec<CustomEmoji>,
    /// Additional metadata attached to a profile as name-value pairs. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#fields)
    #[serde(default = "Vec::new")]
    pub fields: Vec<MetadataField>,
    /// The number of followers for the account. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#followers_count)
    pub followers_count: u64,
    /// The number of accounts the given account is following. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#following_count)
    pub following_count: u64,
    /// Indicates that the account represents a Group actor. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#group)
    #[serde(default)]
    pub group: bool,
    /// An image banner that is shown above the profile and in profile cards. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#header)
    pub header: Url,
    /// A static version of the header. Equal to header if its value is a static
    /// image; different if header is an animated GIF. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#header_static)
    pub header_static: Url,
    /// The ID of the account. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#id)
    pub id: AccountId,
    /// When the most recent status was posted, or `None` if no statuses. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#last_status_at)
    ///
    /// !! TODO parse partial time !!
    ///
    // #[serde(with = "iso8601::option")]
    pub last_status_at: Option<String>,
    /// An extra attribute returned only when an account is silenced. If true,
    /// indicates that the account should be hidden behind a warning screen. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#limited)
    #[serde(default)]
    pub limited: bool,
    /// Whether the account manually approves follow requests. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#locked)
    pub locked: bool,
    /// Indicates that the profile is currently inactive and that its user has
    /// moved to a new account. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#moved)
    pub moved: Option<Box<Account>>,
    /// Whether the local user has opted out of being indexed by search engines. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#noindex)
    #[serde(rename = "noindex")]
    pub no_index: Option<bool>,
    /// The profile’s bio or description. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#note)
    pub note: String,
    /// An extra attribute given from `verify_credentials` giving defaults about
    /// a user
    pub source: Option<Source>,
    /// How many statuses are attached to this account. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#statuses_count)
    pub statuses_count: u64,
    /// An extra attribute returned only when an account is suspended. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#suspended)
    #[serde(default)]
    pub suspended: bool,
    /// The location of the user’s profile page. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#url)
    pub url: Url,
    /// The username of the account, not including domain. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#username)
    pub username: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CredentialAccount {
    /// The data which is in common with all Account queries.
    #[serde(flatten)]
    pub account: Account,
    /// An extra attribute that contains source values to be used with API
    /// methods that [verify credentials](https://docs.joinmastodon.org/methods/accounts/#verify_credentials)
    /// and [update credentials](https://docs.joinmastodon.org/methods/accounts/#update_credentials). See also [the API reference](https://docs.joinmastodon.org/entities/Account/#source)
    pub source: Source,
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
    /// The default post privacy to be used for new statuses. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#source-privacy)
    pub privacy: Option<crate::visibility::Visibility>,
    /// Whether new statuses should be marked sensitive by default. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#source-sensitive)
    #[serde(deserialize_with = "string_or_bool")]
    pub sensitive: bool,
    /// Profile bio, in plain-text instead of in HTML. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#source-note)
    pub note: Option<String>,
    /// Metadata about the account. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#source-fields)
    pub fields: Option<Vec<MetadataField>>,
    /// The default posting language for new statuses. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#source-language)
    pub language: Option<String>,
    /// The number of pending follow requests. See also [the API reference](https://docs.joinmastodon.org/entities/Account/#follow_requests_count)
    pub follow_requests_count: u64,
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

/// Defaults for new posts
#[derive(Builder, Debug, Default, Clone, Serialize, PartialEq, Eq)]
#[builder(build_fn(error = "crate::error::Error"))]
pub struct UpdateSource {
    /// Default post privacy for authored statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    pub privacy: Option<crate::visibility::Visibility>,
    /// Whether to mark authored statuses as sensitive by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    pub sensitive: Option<bool>,
    /// Default language to use for authored statuses
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    pub language: Option<Language>,
}

/// Form data for [`Mastodon::update_credentials()`](https://docs.rs/mastodon-async/latest/mastodon_async/mastodon/struct.Mastodon.html#method.update_credentials).
///
/// See also [the API reference](https://docs.joinmastodon.org/methods/accounts/#form-data-parameters-1).
#[derive(Debug, Builder, Default, Serialize, PartialEq, Eq)]
#[builder(build_fn(error = "crate::error::Error"))]
pub struct Credentials {
    /// The display name to use for the profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option))]
    pub display_name: Option<String>,
    /// The account bio.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option))]
    pub note: Option<String>,
    /// Avatar image
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option))]
    pub avatar: Option<PathBuf>,
    /// Header image
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option))]
    pub header: Option<PathBuf>,
    /// Whether manual approval of follow requests is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// Whether the account has a bot flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
    /// Whether the account should be shown in the profile directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverable: Option<bool>,
    /// Defaults for new posts
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option))]
    pub source: Option<UpdateSource>,
    ///  The profile fields to be set
    #[serde(serialize_with = "fields_attributes_ser::ser")]
    #[builder(setter(custom), default)]
    pub fields_attributes: Vec<MetadataField>,
}

impl CredentialsBuilder {
    /// Set an account attribute.
    pub fn fields_attribute(
        &mut self,
        field: impl Into<String>,
        value: impl Into<String>,
    ) -> &mut Self {
        self.fields_attributes
            .get_or_insert_with(Default::default)
            .push(MetadataField {
                name: field.into(),
                value: value.into(),
            });
        self
    }
}

/// Represents a custom user role that grants permissions.
///
/// See also [the API reference](https://docs.joinmastodon.org/entities/Role/)
///
/// ## Example
/// ```
/// use mastodon_async_entities::account::Role;
/// let example_serialized = r##"{
///     "id": 3,
///     "name": "Owner",
///     "color": "#ff3838",
///     "position": 1000,
///     "permissions": 1,
///     "highlighted": true,
///     "created_at": "2022-09-08T22:48:07.983Z",
///     "updated_at": "2022-09-08T22:48:07.983Z"
/// }"##;
/// let example: Role = serde_json::from_str(example_serialized).unwrap();
/// assert_eq!(example.name, "Owner");
/// assert!(example.permissions.has_administrator());
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Role {
    /// The ID of the Role in the database See also [the API reference](https://docs.joinmastodon.org/entities/Role/#id)
    pub id: i64,
    /// The name of the role. See also [the API reference](https://docs.joinmastodon.org/entities/Role/#name)
    pub name: String,
    /// The hex code assigned to this role. If no hex code is assigned, the
    /// string will be empty. See also [the API reference](https://docs.joinmastodon.org/entities/Role/#color)
    pub color: Color,
    /// An index for the role’s position. The higher the position, the more
    /// priority the role has over other roles. See also [the API reference](https://docs.joinmastodon.org/entities/Role/#position)
    pub position: i64,
    /// A bitmask that represents the sum of all permissions granted to the
    /// role. See also [the API reference](https://docs.joinmastodon.org/entities/Role/#permissions) and
    /// <https://docs.joinmastodon.org/entities/Role/#permission-flags>
    #[serde(with = "role_permissions_serde::numeric_representation::stringified")]
    pub permissions: RolePermissions,
    /// Whether the role is publicly visible as a badge on user profiles. See also [the API reference](https://docs.joinmastodon.org/entities/Role/#highlighted)
    pub highlighted: bool,
    /// The date that the role was created See also [the API reference](https://docs.joinmastodon.org/entities/Role/#created_at)
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// The date that the role was updated. See also [the API reference](https://docs.joinmastodon.org/entities/Role/#updated_at)
    #[serde(with = "iso8601")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// An RGB color as expected by the API. Valid values are the empty string
/// (`Unspecified`) or hexadecimal color codes like `"#C0FF3E"`.
pub enum Color {
    Unspecified,
    Value { red: u8, green: u8, blue: u8 },
}

impl FromStr for Color {
    type Err = ParseIntError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let red = u8::from_str_radix(&text[1..=2], 16)?;
        let green = u8::from_str_radix(&text[3..=4], 16)?;
        let blue = u8::from_str_radix(&text[5..=6], 16)?;
        Ok(Color::Value { red, blue, green })
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ColorVisitor;

        impl<'v> Visitor<'v> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "the empty string or a 6-character hex code prefixed with #"
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v.is_empty() {
                    return Ok(Color::Unspecified);
                }
                if v.len() != 7 {
                    return Err(de::Error::invalid_length(v.len(), &Self));
                }
                let first = v
                    .chars()
                    .next()
                    .expect("string length has already been checked");
                if first != '#' {
                    return Err(de::Error::invalid_value(Unexpected::Char(first), &Self));
                }
                Color::from_str(v).map_err(|_| de::Error::invalid_value(Unexpected::Str(v), &Self))
            }
        }

        deserializer.deserialize_str(ColorVisitor)
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        if let Color::Value { red, green, blue } = self {
            format!("#{red:x}{green:x}{blue:x}")
        } else {
            String::new()
        }
    }
}
impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[classic_bitfield::bitfield_enum]
pub enum RolePermissions {
    /// Users with this permission bypass all permissions.
    ADMINISTRATOR,
    /// Allows users to access Sidekiq and pgHero dashboards.
    DEVOPS,
    /// Allows users to see history of admin actions.
    VIEW_AUDIT_LOG,
    /// Allows users to access the dashboard and various metrics.
    VIEW_DASHBOARD,
    /// Allows users to review reports and perform moderation actions against them.
    MANAGE_REPORTS,
    /// Allows users to block or allow federation with other domains, and control deliverability.
    MANAGE_FEDERATION,
    /// Allows users to change site settings.
    MANAGE_SETTINGS,
    /// Allows users to block e-mail providers and IP addresses.
    MANAGE_BLOCKS,
    /// Allows users to review trending content and update hashtag settings.
    MANAGE_TAXONOMIES,
    /// Allows users to review appeals against moderation actions.
    MANAGE_APPEALS,
    /// Allows users to view other users’ details and perform moderation actions
    /// against them.
    MANAGE_USERS,
    /// Allows users to browse and deactivate invite links.
    MANAGE_INVITES,
    /// Allows users to change server rules.
    MANAGE_RULES,
    /// Allows users to manage announcements on the server.
    MANAGE_ANNOUNCEMENTS,
    /// Allows users to manage custom emojis on the server.
    MANAGE_CUSTOM_EMOJIS,
    /// Allows users to set up webhooks for administrative events.
    MANAGE_WEBHOOKS,
    /// Allows users to invite new people to the server.
    INVITE_USERS,
    /// Allows users to manage and assign roles below theirs.
    MANAGE_ROLES,
    /// Allows users to disable other users’ two-factor authentication, change
    /// their e-mail address, and reset their password.
    MANAGE_USER_ACCESS,
    /// Allows users to delete other users’ data without delay.
    DELETE_USER_DATA,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_parse() {
        let example = r##""#c0ffee""##;
        let color: Color = serde_json::from_str(example).expect("parse");
        if let Color::Value { red, green, blue } = color {
            assert_eq!(red, 0xC0);
            assert_eq!(green, 0xFF);
            assert_eq!(blue, 0xEE);
        }
        assert_eq!(serde_json::to_string(&color).expect("serialize"), example);
        let color: Color = serde_json::from_str(r#""""#).expect("parse");
        assert_eq!(Color::Unspecified, color);
        assert!(color.to_string().is_empty());
    }

    #[test]
    fn test_permissions() {
        assert_eq!(RolePermissions::ADMINISTRATOR, 0x1);
        assert_eq!(RolePermissions::DEVOPS, 0x2);
        assert_eq!(RolePermissions::VIEW_AUDIT_LOG, 0x4);
        assert_eq!(RolePermissions::VIEW_DASHBOARD, 0x8);
        assert_eq!(RolePermissions::MANAGE_REPORTS, 0x10);
        assert_eq!(RolePermissions::MANAGE_FEDERATION, 0x20);
        assert_eq!(RolePermissions::MANAGE_SETTINGS, 0x40);
        assert_eq!(RolePermissions::MANAGE_BLOCKS, 0x80);
        assert_eq!(RolePermissions::MANAGE_TAXONOMIES, 0x100);
        assert_eq!(RolePermissions::MANAGE_APPEALS, 0x200);
        assert_eq!(RolePermissions::MANAGE_USERS, 0x400);
        assert_eq!(RolePermissions::MANAGE_INVITES, 0x800);
        assert_eq!(RolePermissions::MANAGE_RULES, 0x1000);
        assert_eq!(RolePermissions::MANAGE_ANNOUNCEMENTS, 0x2000);
        assert_eq!(RolePermissions::MANAGE_CUSTOM_EMOJIS, 0x4000);
        assert_eq!(RolePermissions::MANAGE_WEBHOOKS, 0x8000);
        assert_eq!(RolePermissions::INVITE_USERS, 0x10000);
        assert_eq!(RolePermissions::MANAGE_ROLES, 0x20000);
        assert_eq!(RolePermissions::MANAGE_USER_ACCESS, 0x40000);
        assert_eq!(RolePermissions::DELETE_USER_DATA, 0x80000);
    }
}
