//! Module containing all info about notifications.

use super::{account::Account, status::Status};
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// A struct containing info about a notification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Notification {
    /// The notification ID.
    pub id: String,
    /// The type of notification.
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    /// The time the notification was created.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// The Account sending the notification to the user.
    pub account: Account,
    /// The Status associated with the notification, if applicable.
    pub status: Option<Status>,
}

/// The type of notification.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NotificationType {
    /// Someone mentioned the application client in another status.
    Mention,
    /// Someone reblogged one of the application client's statuses.
    Reblog,
    /// Someone favourited one of the application client's statuses.
    Favourite,
    /// Someone followed the application client.
    Follow,
    /// A poll the application client previously voted in has ended.
    Poll,
}
