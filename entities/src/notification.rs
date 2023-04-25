//! Module containing all info about notifications.

use crate::{account::Account, admin::Report, status::Status, NotificationId};
use derive_is_enum_variant::is_enum_variant;
use enumset::{EnumSet, EnumSetType};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};
use std::collections::BTreeMap;
use std::str::FromStr;
use strum::{Display, EnumString};
use time::{serde::iso8601, OffsetDateTime};

/// Represents a notification of an event relevant to the user.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Notification/)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Notification {
    /// The id of the notification in the database.
    pub id: NotificationId,
    /// The type of event that resulted in the notification..
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    /// The timestamp of the notification.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// The account that performed the action that generated the notification..
    pub account: Account,
    /// Status that was the object of the notification. Attached when type of
    /// the notification is `favourite`, `reblog`, `status`, `mention`, `poll`,
    /// or `update`.
    pub status: Option<Status>,
    /// Report that was the object of the notification. Attached when type of
    /// the notification is `admin.report`.
    pub report: Option<Report>,
}

/// The type of notification.
#[derive(
    Debug,
    Deserialize,
    Serialize,
    Display,
    EnumString,
    EnumSetType,
    PartialOrd,
    Ord,
    is_enum_variant,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum NotificationType {
    /// Someone mentioned you in their status.
    Mention,
    /// Someone you enabled notifications for has posted a status.
    Status,
    /// Someone boosted one of your statuses.
    Reblog,
    /// Someone favourited one of your statuses.
    Favourite,
    /// Someone followed you.
    Follow,
    /// Someone requested to follow you.
    FollowRequest,
    /// A poll you have voted in or created has ended.
    Poll,
    /// A status you interacted with has been edited.
    Update,
    /// Someone signed up (admins only).
    #[serde(rename = "admin.sign_up")]
    #[strum(serialize = "admin.sign_up")]
    AdminSignUp,
    /// A new report has been filed (admins only).
    #[serde(rename = "admin.report")]
    #[strum(serialize = "admin.report")]
    AdminReport,
}

/// Serialization helper for contexts where an `EnumSet<NotificationType>` gets handled as `BTreeMap<String, bool>`.
/// Invoke with an `#[serde_as(as = "NotificationTypeMap")]` attribute.
pub struct NotificationTypeMap;

impl SerializeAs<EnumSet<NotificationType>> for NotificationTypeMap {
    fn serialize_as<S>(source: &EnumSet<NotificationType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        source
            .iter()
            .map(|notification_type| (notification_type.to_string(), true))
            .collect::<BTreeMap<String, bool>>()
            .serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, EnumSet<NotificationType>> for NotificationTypeMap {
    fn deserialize_as<D>(deserializer: D) -> Result<EnumSet<NotificationType>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(BTreeMap::<String, bool>::deserialize(deserializer)?
            .into_iter()
            .flat_map(|(k, v)| NotificationType::from_str(&k).ok().filter(|_| v))
            .collect::<EnumSet<_>>())
    }
}
