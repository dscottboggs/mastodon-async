use serde::{Deserialize, Serialize};

use crate::AccountId;

/// Represents a `mention` used in a status
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Mention {
    /// URL of user's profile (can be remote)
    pub url: String,
    /// The username of the account
    pub username: String,
    /// Equals username for local users, includes `@domain` for remote ones
    pub acct: String,
    /// Account ID
    pub id: AccountId,
}
