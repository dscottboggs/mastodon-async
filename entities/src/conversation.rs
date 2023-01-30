use serde::{Deserialize, Serialize};

use crate::{account::Account, status::Status, ConversationId};

/// Represents a conversation with "direct message" visibility.
///
/// See also [the API documentation](https://docs.joinmastodon.org/entities/Conversation/)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Conversation {
    /// The ID of the conversation in the database.
    pub id: ConversationId,
    /// Is the conversation currently marked as unread?
    pub unread: bool,
    ///  Participants in the conversation.
    pub accounts: Vec<Account>,
    /// The last status in the conversation.
    pub last_status: Option<Status>,
}
