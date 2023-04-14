use std::collections::HashMap;
use valuable::Valuable;

#[derive(Clone, Debug, Valuable)]
pub(crate) struct Response<'a> {
    url: &'a str,
    status: StatusCode<'a>,
    headers: HashMap<String, String>,
}

impl<'a> From<&'a reqwest::Response> for Response<'a> {
    fn from(response: &'a reqwest::Response) -> Self {
        let url = response.url().as_str();
        let status = StatusCode::from(&response.status());
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    if v.is_sensitive() {
                        // don't log sensitive headers
                        "<redacted>".to_string()
                    } else {
                        // don't log if the header contains non-visible characters
                        v.to_str().unwrap_or("<non-visible-chars>").to_string()
                    },
                )
            })
            .collect();
        Self {
            url,
            status,
            headers,
        }
    }
}

#[derive(Clone, Debug, Valuable)]
pub(crate) struct StatusCode<'a> {
    code: u16,
    reason: &'a str,
}

impl<'a> From<&reqwest::StatusCode> for StatusCode<'a> {
    fn from(status: &reqwest::StatusCode) -> Self {
        Self {
            code: status.as_u16(),
            reason: status.canonical_reason().unwrap_or("unknown"),
        }
    }
}

/// A macro to create a `valuable::Value` from a type defined in this module
#[macro_export]
macro_rules! as_value {
    ($response:ident, $type_name:tt) => {
        valuable::Valuable::as_value(&$crate::helpers::tracing::$type_name::from(&$response))
    };
}
