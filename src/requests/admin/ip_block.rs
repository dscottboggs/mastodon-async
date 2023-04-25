use crate::entities::admin::ip_block::Severity;
use ipnet::IpNet;
use mastodon_async_derive::request_builder;
use serde_with::{serde_as, DurationSeconds};
use time::Duration;

/// Create a new IP range block.
#[serde_as]
#[request_builder]
pub struct AddIpBlockRequest {
    /// The IP address range that is not allowed to federate.
    pub ip: Option<IpNet>,
    /// The policy associated with this IP block.
    pub severity: Severity,
    /// The recorded reason for this IP block.
    pub comment: Option<String>,
    /// The number of seconds in which this IP block will expire.
    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    pub expires_in: Option<Duration>,
}

/// Update an existing IP range block.
/// Differs from [`AddIpBlockRequest`] only in that all parameters are optional.
#[serde_as]
#[request_builder]
pub struct UpdateIpBlockRequest {
    ip: Option<IpNet>,
    /// The policy to apply to this IP range.
    severity: Option<Severity>,
    /// The reason for this IP block.
    comment: Option<String>,
    /// The number of seconds in which this IP block will expire.
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
