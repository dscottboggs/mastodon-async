//! module containing information about a finished report of a user.
use serde::{Deserialize, Serialize};

use crate::ReportId;

/// A struct containing info about a report.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Report {
    /// The ID of the report.
    pub id: ReportId,
    /// The action taken in response to the report.
    pub action_taken: String,
}
