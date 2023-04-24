use crate::entities::admin::ip_block::Severity;
use derive_builder::Builder;
use ipnet::IpNet;
use mastodon_async_derive::RequestBuilder;
use serde_with::{serde_as, skip_serializing_none, DurationSeconds};
use time::Duration;

/// Create a new IP range block.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Builder, RequestBuilder)]
#[builder(
    derive(Debug, PartialEq),
    custom_constructor,
    build_fn(private, name = "try_build"),
    setter(into, strip_option)
)]
pub struct AddIpBlockRequest {
    /// The IP address range that is not allowed to federate.
    #[builder(default)]
    pub ip: Option<IpNet>,
    /// The policy associated with this IP block.
    #[builder(private)]
    pub severity: Severity,
    /// The recorded reason for this IP block.
    #[builder(default)]
    pub comment: Option<String>,
    /// The number of seconds in which this IP block will expire.
    #[builder(default)]
    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    pub expires_in: Option<Duration>,
}

/// Update an existing IP range block.
/// Differs from [`AddIpBlockRequest`] only in that all parameters are optional.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Builder, RequestBuilder)]
#[builder(
    derive(Debug, PartialEq),
    custom_constructor,
    build_fn(private, name = "try_build"),
    setter(into, strip_option)
)]
pub struct UpdateIpBlockRequest {
    #[builder(default)]
    ip: Option<IpNet>,
    /// The policy to apply to this IP range.
    #[builder(default)]
    severity: Option<Severity>,
    /// The reason for this IP block.
    #[builder(default)]
    comment: Option<String>,
    /// The number of seconds in which this IP block will expire.
    #[builder(default)]
    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    expires_in: Option<Duration>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ipnet::IpNet;
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn test_serialize_add_request() {
        let request = AddIpBlockRequest::builder(Severity::SignUpRequiresApproval)
            .ip(IpNet::from_str("192.168.0.0/16").unwrap())
            .comment("test comment")
            .expires_in(Duration::seconds(86400))
            .build();
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(
            ser,
            r#"{"ip":"192.168.0.0/16","severity":"sign_up_requires_approval","comment":"test comment","expires_in":86400}"#
        );
    }

    #[test]
    fn test_serialize_update_request() {
        let request = UpdateIpBlockRequest::builder()
            .severity(Severity::NoAccess)
            .build();
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(ser, r#"{"severity":"no_access"}"#);
    }
}
