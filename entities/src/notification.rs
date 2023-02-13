//! Module containing all info about notifications.

use crate::{admin::Report, NotificationId};

use super::{account::Account, status::Status};
use derive_is_enum_variant::is_enum_variant;
use serde::{Deserialize, Serialize};
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
    pub notification_type: Type,
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
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, is_enum_variant)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    /// Someone mentioned you in their status
    Mention,
    /// Someone you enabled notifications for has posted a status
    Status,
    /// Someone boosted one of your statuses
    Reblog,
    /// Someone favourited one of your statuses
    Favourite,
    /// Someone followed you
    Follow,
    /// Someone requested to follow you
    FollowRequest,
    /// A poll you have voted in or created has ended
    Poll,
    /// A status you interacted with has been edited
    Update,
    /// Someone signed up (optionally sent to admins)
    #[serde(rename = "admin.sign_up")]
    SignUp,
    /// A new report has been filed
    #[serde(rename = "admin.report")]
    Report,
}
