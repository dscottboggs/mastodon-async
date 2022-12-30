use crate::{notification::Notification, status::Status};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Events that come from the /streaming/user API call
pub enum Event {
    /// Update event
    Update(Status),
    /// Notification event
    Notification(Notification),
    /// Delete event
    Delete(String),
    /// FiltersChanged event
    FiltersChanged,
}
