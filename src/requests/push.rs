use mastodon_async_entities::push::Alerts;

use crate::entities::push::{add_subscription, update_data};

/// Container for the key & auth strings for an AddPushRequest
///
/// // Example
///
/// ```
/// use mastodon_async::requests::Keys;
///
/// let keys = Keys::new("anetohias===", "oeatssah=");
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Keys {
    pub(crate) p256dh: String,
    pub(crate) auth: String,
}

impl Keys {
    /// Create the `Keys` container
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::requests::Keys;
    ///
    /// let keys = Keys::new("anetohias===", "oeatssah=");
    /// ```
    pub fn new(p256dh: &str, auth: &str) -> Keys {
        Keys {
            p256dh: p256dh.to_string(),
            auth: auth.to_string(),
        }
    }
}

/// Builder to pass to the Mastodon::add_push_subscription method
///
/// // Example
///
/// ```no_run
/// use mastodon_async::{
///     entities::push::AlertsBuilder,
///     Mastodon,
///     Data,
///     requests::{AddPushRequest, Keys}
/// };
///
/// tokio_test::block_on(async {
///     let data = Data::default();
///     let client = Mastodon::from(data);
///
///     let keys = Keys::new("stahesuahoei293ise===", "tasecoa,nmeozka==");
///     let mut request = AddPushRequest::new("http://example.com/push/endpoint", &keys);
///     request.alerts(
///         AlertsBuilder::default()
///             .follow(true)
///             .reblog(true)
///             .build()
///             .unwrap()
///     );
///     client.add_push_subscription(&request).await.unwrap();
/// });
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AddPushRequest {
    endpoint: String,

    p256dh: String,
    auth: String,

    alerts: Alerts,
}

impl AddPushRequest {
    /// Construct a new AddPushRequest
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::requests::{AddPushRequest, Keys};
    /// let keys = Keys::new("abcdef===", "foobar==");
    /// let push_endpoint = "https://example.com/push/endpoint";
    /// let request = AddPushRequest::new(push_endpoint, &keys);
    /// ```
    pub fn new(endpoint: &str, keys: &Keys) -> AddPushRequest {
        AddPushRequest {
            endpoint: endpoint.to_string(),
            p256dh: keys.p256dh.clone(),
            auth: keys.auth.clone(),
            ..Default::default()
        }
    }

    /// Set the alerts which should be requested to be notified by this request.
    pub fn alerts(&mut self, alerts: Alerts) -> &mut Self {
        self.alerts = alerts;
        self
    }

    /// Build the form.
    pub fn build(&self) -> add_subscription::Form {
        use crate::entities::push::add_subscription::{Data, Form, Keys, Subscription};
        let mut form = Form {
            subscription: Subscription {
                endpoint: self.endpoint.clone(),
                keys: Keys {
                    p256dh: self.p256dh.clone(),
                    auth: self.auth.clone(),
                },
            },
            data: None,
        };

        if self.alerts.is_some() {
            form.data = Some(Data {
                alerts: Some(self.alerts),
            });
        }

        form
    }
}

/// Builder to pass to the Mastodon::update_push_data method
///
/// // Example
///
/// ```no_run
/// use mastodon_async::{
///     entities::push::AlertsBuilder,
///     Mastodon,
///     Data,
///     requests::UpdatePushRequest
/// };
///
/// let data = Data::default();
/// let client = Mastodon::from(data);
///
/// let mut request = UpdatePushRequest::new("foobar");
/// request.alerts(
///     AlertsBuilder::default()
///         .follow(true)
///         .reblog(true)
///         .build()
///         .unwrap()
/// );
/// tokio_test::block_on(async {
///     client.update_push_data(&request).await.unwrap();
/// });
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
pub struct UpdatePushRequest {
    id: String,
    alerts: Alerts,
}

impl UpdatePushRequest {
    /// Construct a new UpdatePushRequest
    ///
    /// // Example
    ///
    /// ```
    /// let request = mastodon_async::requests::UpdatePushRequest::new("some-id");
    /// ```
    pub fn new(id: &str) -> UpdatePushRequest {
        UpdatePushRequest {
            id: id.to_string(),
            ..Default::default()
        }
    }

    /// Set alerts which should be enabled or disabled by this update.
    pub fn alerts(&mut self, alerts: Alerts) -> &mut Self {
        self.alerts = alerts;
        self
    }

    /// Build the form from the update
    pub fn build(&self) -> update_data::Form {
        use crate::entities::push::update_data::{Data, Form};

        let mut form = Form {
            id: self.id.clone(),
            ..Default::default()
        };

        if self.alerts.is_some() {
            form.data = Data {
                alerts: Some(self.alerts),
            };
        }
        form
    }
}

#[cfg(test)]
mod tests {
    use mastodon_async_entities::push::AlertsBuilder;

    use super::*;
    use crate::entities::push::{add_subscription, update_data, Alerts};

    #[test]
    fn test_keys_new() {
        let keys = Keys::new("anetohias===", "oeatssah=");
        assert_eq!(
            keys,
            Keys {
                p256dh: "anetohias===".to_string(),
                auth: "oeatssah=".to_string()
            }
        );
    }

    #[test]
    fn test_add_push_request_new() {
        let endpoint = "https://example.com/push/endpoint";
        let keys = Keys::new("anetohias===", "oeatssah=");
        let req = AddPushRequest::new(endpoint, &keys);
        assert_eq!(
            req,
            AddPushRequest {
                endpoint: "https://example.com/push/endpoint".to_string(),
                p256dh: "anetohias===".to_string(),
                auth: "oeatssah=".to_string(),
                ..Default::default()
            }
        );
    }

    macro_rules! alerts_builder_test {
        ($name:ident, $set:ident $(; $rest_names:ident, $rest_set:ident)*;) => {
            #[test]
            fn $name() {
                let endpoint = "https://example.com/push/endpoint";
                let keys = Keys::new("anetohias===", "oeatssah=");
                let mut req = AddPushRequest::new(endpoint, &keys);
                req.alerts(AlertsBuilder::default().$set(true).build().unwrap());
                assert_eq!(
                    req,
                    AddPushRequest {
                        endpoint: "https://example.com/push/endpoint".to_string(),
                        p256dh: "anetohias===".to_string(),
                        auth: "oeatssah=".to_string(),
                        alerts: Alerts {
                            $set: Some(true),
                            ..Default::default()
                        }
                    }
                );
            }

            alerts_builder_test!($($rest_names, $rest_set;)*);
        };
        () => {};
    }
    alerts_builder_test!(
        test_add_push_request_follow, follow;
        test_add_push_request_favourite, favourite;
        test_add_push_request_reblog, reblog;
        test_add_push_request_mention, mention;
    );

    #[test]
    fn test_add_push_request_build() {
        let endpoint = "https://example.com/push/endpoint";
        let keys = Keys::new("anetohias===", "oeatssah=");
        let mut req = AddPushRequest::new(endpoint, &keys);
        req.alerts(
            AlertsBuilder::default()
                .follow(true)
                .reblog(true)
                .build()
                .unwrap(),
        );
        let form = req.build();
        assert_eq!(
            form,
            add_subscription::Form {
                subscription: add_subscription::Subscription {
                    endpoint: "https://example.com/push/endpoint".to_string(),
                    keys: add_subscription::Keys {
                        p256dh: "anetohias===".to_string(),
                        auth: "oeatssah=".to_string(),
                    },
                },
                data: Some(add_subscription::Data {
                    alerts: Some(Alerts {
                        follow: Some(true),
                        reblog: Some(true),
                        ..Default::default()
                    }),
                }),
            }
        );
    }

    #[test]
    fn test_update_push_request_new() {
        let req = UpdatePushRequest::new("some-id");
        assert_eq!(
            req,
            UpdatePushRequest {
                id: "some-id".to_string(),
                ..Default::default()
            }
        );
    }

    macro_rules! test_update_push_request {
        ($name:ident, $set:ident $(; $rest_names:ident, $rest_set:ident)*;) => {
            #[test]
            fn $name() {
                let mut req = UpdatePushRequest::new("some-id");
                req.alerts(AlertsBuilder::default().$set(true).build().unwrap());
                assert_eq!(
                    req,
                    UpdatePushRequest {
                        id: "some-id".to_string(),
                        alerts: Alerts {
                            $set: Some(true),
                            ..Default::default()
                        }
                    }
                );
            }

            test_update_push_request!($($rest_names, $rest_set;)*);
        };
        () => {}
    }

    test_update_push_request! {
        test_update_push_request_follow, follow;
        test_update_push_request_favourite, favourite;
        test_update_push_request_reblog, reblog;
        test_update_push_request_mention, mention;

    }

    #[test]
    fn test_update_push_request_build_no_flags() {
        let req = UpdatePushRequest::new("some-id");
        let form = req.build();
        assert_eq!(
            form,
            update_data::Form {
                id: "some-id".to_string(),
                data: update_data::Data { alerts: None },
            }
        );
    }

    #[test]
    fn test_update_push_request_build() {
        let mut req = UpdatePushRequest::new("some-id");
        req.alerts(Alerts {
            favourite: Some(false),
            ..Default::default()
        });
        let form = req.build();
        assert_eq!(
            form,
            update_data::Form {
                id: "some-id".to_string(),
                data: update_data::Data {
                    alerts: Some(Alerts {
                        favourite: Some(false),
                        ..Default::default()
                    }),
                },
            }
        );
    }
}
