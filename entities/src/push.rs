use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::SubscriptionId;

/// Represents the `alerts` key of the `Subscription` object
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, Builder)]
#[builder(build_fn(error = "crate::error::Error"), default)]
pub struct Alerts {
    /// flag for follow alerts
    #[serde(default)]
    #[builder(setter(strip_option))]
    pub follow: Option<bool>,
    /// flag for favourite alerts
    #[serde(default)]
    #[builder(setter(strip_option))]
    pub favourite: Option<bool>,
    /// flag for reblog alerts
    #[serde(default)]
    #[builder(setter(strip_option))]
    pub reblog: Option<bool>,
    /// flag for mention alerts
    #[serde(default)]
    #[builder(setter(strip_option))]
    pub mention: Option<bool>,
    /// Receive a push notification when a subscribed account posts a status?
    #[serde(default)]
    #[builder(setter(strip_option))]
    pub status: Option<bool>,
    /// Receive a push notification when someone has requested to followed you?
    #[serde(default)]
    #[builder(setter(strip_option))]
    pub follow_request: Option<bool>,
    /// Receive a push notification when a poll you voted in or created has ended?
    #[serde(default)]
    #[builder(setter(strip_option))]
    pub poll: Option<bool>,
    /// Receive a push notification when a status you interacted with has been edited?
    #[serde(default)]
    #[builder(setter(strip_option))]
    pub update: Option<bool>,
    /// Admin-related alerts settings
    #[serde(flatten, default)]
    #[builder(setter(custom))]
    pub admin: AdminAlerts,
}
/// Admin-related alerts settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AdminAlerts {
    /// Receive a push notification when a new user has signed up?
    #[serde(default, rename = "admin.sign_up")]
    pub sign_up: Option<bool>,
    /// Receive a push notification when a new report has been filed?
    #[serde(default, rename = "admin.report")]
    pub report: Option<bool>,
}

impl Alerts {
    pub fn sign_up(&mut self, v: bool) -> &mut Self {
        self.admin.sign_up = Some(v);
        self
    }
    pub fn report(&mut self, v: bool) -> &mut Self {
        self.admin.report = Some(v);
        self
    }
    /// True if every field is `None`.
    pub fn is_none(&self) -> bool {
        self.admin.report.is_none()
            && self.admin.sign_up.is_none()
            && self.favourite.is_none()
            && self.follow.is_none()
            && self.follow_request.is_none()
            && self.reblog.is_none()
            && self.mention.is_none()
            && self.status.is_none()
            && self.poll.is_none()
            && self.update.is_none()
    }

    /// True if any field is set.
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}
/// Represents a subscription to the push streaming server.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/WebPushSubscription/)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subscription {
    /// The ID of the Web Push subscription in the database.
    pub id: SubscriptionId,
    /// Where push alerts will be sent to.
    pub endpoint: String,
    /// The streaming server’s VAPID key.
    pub server_key: String,
    /// Which alerts should be delivered to the endpoint.
    pub alerts: Alerts,
}

pub mod add_subscription {
    use serde::Serialize;

    use super::Alerts;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
    pub struct Form {
        pub subscription: Subscription,
        pub data: Option<Data>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
    pub struct Subscription {
        pub endpoint: String,
        pub keys: Keys,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
    pub struct Keys {
        pub p256dh: String,
        pub auth: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
    pub struct Data {
        pub alerts: Option<Alerts>,
    }
}

pub mod update_data {
    use serde::Serialize;

    use super::Alerts;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
    pub struct Data {
        pub alerts: Option<Alerts>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
    pub struct Form {
        pub id: String,
        pub data: Data,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_subscription() {
        let example = r#"{
          "id": "328183",
          "endpoint": "https://yourdomain.example/listener",
          "alerts": {
            "follow": false,
            "favourite": false,
            "reblog": false,
            "mention": true,
            "poll": false
          },
          "server_key": "BCk-QqERU0q-CfYZjcuB6lnyyOYfJ2AifKqfeGIm7Z-HiTU5T9eTG5GxVA0_OH5mMlI4UkkDTpaZwozy0TzdZ2M="
        }"#;
        let subject: Subscription = serde_json::from_str(example).unwrap();
        assert_eq!(subject.id, SubscriptionId::new("328183"));
        assert_eq!(subject.endpoint, "https://yourdomain.example/listener");
        assert!(!subject.alerts.follow.unwrap());
        assert!(!subject.alerts.favourite.unwrap());
        assert!(!subject.alerts.reblog.unwrap());
        assert!(subject.alerts.mention.unwrap());
        assert!(!subject.alerts.poll.unwrap());
        assert!(subject.alerts.admin.sign_up.is_none());
        assert!(subject.alerts.admin.report.is_none());
        assert!(subject.alerts.status.is_none());
        assert!(subject.alerts.follow_request.is_none());
        assert!(subject.alerts.update.is_none());
        assert_eq!(subject.server_key, "BCk-QqERU0q-CfYZjcuB6lnyyOYfJ2AifKqfeGIm7Z-HiTU5T9eTG5GxVA0_OH5mMlI4UkkDTpaZwozy0TzdZ2M=");
    }
}
