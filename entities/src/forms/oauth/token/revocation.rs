use serde::{Deserialize, Serialize};

use crate::{ClientId, ClientSecret, OAuthToken};

/// Form to be submitted when revoking an OAuth token.
///
/// See also the [API Documentation](https://docs.joinmastodon.org/methods/oauth/#revoke)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Revocation {
    client_id: ClientId,
    client_secret: ClientSecret,
    token: OAuthToken,
}

impl Revocation {
    pub fn new(client_id: ClientId, client_secret: ClientSecret, token: OAuthToken) -> Self {
        Self {
            client_id,
            client_secret,
            token,
        }
    }
}
