//! module containing information about a finished report of a user.

use serde::{Deserialize, Serialize};

/// A struct containing info about a report.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Report {
    /// The ID of the report.
    pub id: ReportId,
    /// The action taken in response to the report.
    pub action_taken: String,
}

/// Wrapper type for a report ID string
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct ReportId(String);

impl AsRef<str> for ReportId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
