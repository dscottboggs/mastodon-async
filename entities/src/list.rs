use serde::{Deserialize, Serialize};

/// Used for ser/de of list resources
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct List {
    id: ListId,
    title: String,
}

/// Wrapper type for a list ID string
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct ListId(String);

impl AsRef<str> for ListId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ListId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}
