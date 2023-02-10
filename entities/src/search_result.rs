//! A module containing info relating to a search result.

use serde::{Deserialize, Serialize};

use super::{
    prelude::{Account, Status},
    status::Tag,
};

/// Represents the results of a search.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Search/)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResult {
    /// An array of matched Accounts.
    pub accounts: Vec<Account>,
    /// An array of matched Statuses.
    pub statuses: Vec<Status>,
    /// An array of matched hashtags, as `Tag` objects.
    pub hashtags: Vec<Tag>,
}
