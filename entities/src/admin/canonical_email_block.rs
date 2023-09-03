use serde::{Deserialize, Serialize};

use crate::CanonicalEmailBlockId;

/// Represents a canonical email block (hashed).
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Admin_CanonicalEmailBlock/)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalEmailBlock {
    /// The ID of the email block in the database.
    pub id: CanonicalEmailBlockId,
    /// The SHA256 hash of the canonical email address.
    pub canonical_email_hash: String,
}

impl PartialOrd for CanonicalEmailBlock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CanonicalEmailBlock {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.canonical_email_hash.cmp(&other.canonical_email_hash)
    }
}

#[cfg(test)]
mod tests {
    use crate::serde_value_test;

    use super::*;

    serde_value_test!(test_example(CanonicalEmailBlock): r#"{
    	"id": "2",
    	"canonical_email_hash": "b344e55d11b3fc25d0d53194e0475838bf17e9be67ce3e6469956222d9a34f9c"
    }"#);
}
