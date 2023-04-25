use derive_is_enum_variant::is_enum_variant;
use enumset::EnumSet;
use mastodon_async_derive::request_builder;
use mastodon_async_entities::notification::{NotificationType, NotificationTypeMap};
use serde_with::{base64::Base64, serde_as};

/// Request for creating a new push subscription.
#[request_builder]
pub struct AddPushSubscriptionRequest {
    /// The subscription itself.
    pub subscription: PushRequestSubscription,
    /// Push request preferences.
    pub data: Option<PushRequestData>,
}

/// Request for updating an existing push subscription.
#[request_builder]
pub struct UpdatePushSubscriptionRequest {
    /// Push request preferences.
    pub data: Option<PushRequestData>,
}

/// Authorization and endpoint info for a new push subscription.
#[request_builder]
pub struct PushRequestSubscription {
    /// The endpoint URL that is called when a notification event occurs.
    pub endpoint: String,
    /// Auth information.
    pub keys: PushRequestKeys,
}

/// Authorization info for a new push subscription.
#[request_builder]
#[serde_as]
pub struct PushRequestKeys {
    /// User agent public key: from an ECDH keypair using the `prime256v1` curve.
    #[serde_as(as = "Base64")]
    pub p256dh: Vec<u8>,
    /// Auth secret: 16 bytes of random data.
    #[serde_as(as = "Base64")]
    pub auth: Vec<u8>,
}

/// Push request preferences.
#[request_builder]
#[serde_as]
#[allow(missing_copy_implementations)]
pub struct PushRequestData {
    /// Which events to receive push notifications for.
    #[serde_as(as = "Option<NotificationTypeMap>")]
    pub alerts: Option<EnumSet<NotificationType>>,
    /// Which users to receive push notifications for.
    pub policy: Option<PushRequestPolicy>,
}

/// Controls which users you receive push notifications for.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, is_enum_variant,
)]
#[serde(rename_all = "snake_case")]
pub enum PushRequestPolicy {
    /// Receive push notifications for all users.
    All,
    /// Receive push notifications for users you follow.
    Followed,
    /// Receive push notifications for users who follow you.
    Follower,
    /// Do not receive push notifications.
    None,
}

#[cfg(test)]
mod tests {
    use super::*;
    use enumset::enum_set;

    #[test]
    fn test_serialize_create_request() {
        let request = AddPushSubscriptionRequest::builder(
            PushRequestSubscription::builder(
                "https://yourdomain.example/listener",
                PushRequestKeys::builder(
                    vec![
                        0x97, 0x4a, 0x4c, 0x76, 0x50, 0x55, 0x49, 0x40, 0x9b, 0x68, 0x64, 0x45,
                        0x5f, 0x22, 0x7d, 0xbb, 0x0e, 0x27, 0x75, 0x10, 0xf8, 0x61, 0x40, 0x82,
                        0x8e, 0xf7, 0xa4, 0xc7, 0x24, 0x46, 0x93, 0xd9, 0x92, 0x63, 0x5d, 0xdb,
                        0xd8, 0xb1, 0x4e, 0x20, 0x95, 0xb3, 0x66, 0x30, 0xb8, 0x76, 0x0d, 0x56,
                        0x11, 0xce, 0xe4, 0x04, 0x2b, 0xd5, 0x44, 0xf6, 0x97, 0x98, 0xda, 0x20,
                        0x20, 0x41, 0xb7, 0xab,
                    ],
                    vec![
                        0x2b, 0x44, 0x1b, 0x30, 0xc5, 0x3b, 0x48, 0x37, 0xa9, 0xcd, 0x0c, 0x91,
                        0x6e, 0xbb, 0x26, 0x25,
                    ],
                )
                .build(),
            )
            .build(),
        )
        .data(
            PushRequestData::builder()
                .alerts(enum_set!(
                    NotificationType::Mention | NotificationType::AdminReport
                ))
                .policy(PushRequestPolicy::All)
                .build(),
        )
        .build();
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(
            ser,
            r#"{"subscription":{"endpoint":"https://yourdomain.example/listener","keys":{"p256dh":"l0pMdlBVSUCbaGRFXyJ9uw4ndRD4YUCCjvekxyRGk9mSY13b2LFOIJWzZjC4dg1WEc7kBCvVRPaXmNogIEG3qw==","auth":"K0QbMMU7SDepzQyRbrsmJQ=="}},"data":{"alerts":{"admin.report":true,"mention":true},"policy":"all"}}"#
        );
    }

    #[test]
    fn test_serialize_update_request() {
        let request = UpdatePushSubscriptionRequest::builder()
            .data(
                PushRequestData::builder()
                    .alerts(enum_set!(
                        NotificationType::Mention
                            | NotificationType::Reblog
                            | NotificationType::Favourite
                            | NotificationType::Follow
                    ))
                    .policy(PushRequestPolicy::All)
                    .build(),
            )
            .build();
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(
            ser,
            r#"{"data":{"alerts":{"favourite":true,"follow":true,"mention":true,"reblog":true},"policy":"all"}}"#
        );
    }
}
