use crate::{
    notification::{NotificationType, NotificationTypeMap},
    VapidKey, WebPushSubscriptionId,
};
use enumset::EnumSet;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Represents a subscription to the push streaming server.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/WebPushSubscription/)
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebPushSubscription {
    /// The ID of the Web Push subscription in the database.
    pub id: WebPushSubscriptionId,
    /// Where push alerts will be sent to.
    pub endpoint: String,
    /// The streaming server’s VAPID key.
    pub server_key: VapidKey,
    /// Which alerts should be delivered to the endpoint.
    #[serde_as(as = "NotificationTypeMap")]
    pub alerts: EnumSet<NotificationType>,
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
            "poll": false,
            "admin.report": true
          },
          "server_key": "BCk-QqERU0q-CfYZjcuB6lnyyOYfJ2AifKqfeGIm7Z-HiTU5T9eTG5GxVA0_OH5mMlI4UkkDTpaZwozy0TzdZ2M="
        }"#;
        let subject: WebPushSubscription = serde_json::from_str(example).unwrap();
        assert_eq!(subject.id, WebPushSubscriptionId::new("328183"));
        assert_eq!(subject.endpoint, "https://yourdomain.example/listener");
        assert!(!subject.alerts.contains(NotificationType::Follow));
        assert!(!subject.alerts.contains(NotificationType::Favourite));
        assert!(!subject.alerts.contains(NotificationType::Reblog));
        assert!(subject.alerts.contains(NotificationType::Mention));
        assert!(!subject.alerts.contains(NotificationType::Poll));
        assert!(!subject.alerts.contains(NotificationType::AdminSignUp));
        assert!(subject.alerts.contains(NotificationType::AdminReport));
        assert!(!subject.alerts.contains(NotificationType::Status));
        assert!(!subject.alerts.contains(NotificationType::Follow));
        assert!(!subject.alerts.contains(NotificationType::Update));
        assert_eq!(subject.server_key, "BCk-QqERU0q-CfYZjcuB6lnyyOYfJ2AifKqfeGIm7Z-HiTU5T9eTG5GxVA0_OH5mMlI4UkkDTpaZwozy0TzdZ2M=");
    }
}
