use crate::{notification::Notification, status::Status};
use derive_is_enum_variant::is_enum_variant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, is_enum_variant)]
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
