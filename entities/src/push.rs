use serde::{Deserialize, Serialize};

use crate::SubscriptionId;

/// Represents the `alerts` key of the `Subscription` object
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Alerts {
    /// flag for follow alerts
    pub follow: Option<bool>,
    /// flag for favourite alerts
    pub favourite: Option<bool>,
    /// flag for reblog alerts
    pub reblog: Option<bool>,
    /// flag for mention alerts
    pub mention: Option<bool>,
}

/// Represents a new Push subscription
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subscription {
    /// The `id` of the subscription
    pub id: SubscriptionId,
    /// The endpoint of the subscription
    pub endpoint: String,
    /// The server key of the subscription
    pub server_key: String,
    /// The status of the alerts for this subscription
    pub alerts: Option<Alerts>,
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
