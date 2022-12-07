use serde::{Deserialize, Serialize};

/// Used for ser/de of list resources
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct List {
    id: String,
    title: String,
}
