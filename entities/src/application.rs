use serde::{Deserialize, Serialize};

use crate::VapidKey;

/// Represents an application that interfaces with the REST API to access
/// accounts or post statuses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Application {
    /// The name of your application.
    pub name: String,
    /// The website associated with your application
    pub website: Option<String>,
    /// Used for Push Streaming API. Returned with POST /api/v1/apps. Equivalent
    /// to [`Subscription::server_key`](crate::push::Subscription::server_key)
    pub vapid_key: Option<VapidKey>,
    /// Client ID key, to be used for obtaining OAuth tokens
    pub client_id: Option<String>,
    ///  Client secret key, to be used for obtaining OAuth tokens
    pub client_secret: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let example = r#"{
  "name": "test app",
  "website": null,
  "vapid_key": "BCk-QqERU0q-CfYZjcuB6lnyyOYfJ2AifKqfeGIm7Z-HiTU5T9eTG5GxVA0_OH5mMlI4UkkDTpaZwozy0TzdZ2M="
}"#;
        let app: Application = serde_json::from_str(example).expect("deserialize");
        assert_eq!(app.name, "test app")
    }
}
