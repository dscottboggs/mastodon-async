//! Module containing all info about notifications.

use super::{account::Account, status::Status};
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// A struct containing info about a notification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Notification {
    /// The notification ID.
    pub id: NotificationId,
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

/// Wrapper type for a notification ID string
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct NotificationId(String);

impl AsRef<str> for NotificationId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl NotificationId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
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
    /// Someone asked to followed the application client when follow requests must be approved.
    #[serde(rename = "follow_request")]
    FollowRequest,
    /// A poll the application client previously voted in has ended.
    Poll,
}
