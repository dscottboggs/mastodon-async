use serde::{Deserialize, Serialize};

use crate::ListId;

/// Used for ser/de of list resources
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct List {
    id: ListId,
    title: String,
}
