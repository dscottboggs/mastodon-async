/// Data structure for the MastodonClient::add_push_subscription method
pub use self::push::{AddPushRequest, Keys, UpdatePushRequest};
/// Data structure for the MastodonClient::statuses method
pub use self::statuses::StatusesRequest;

mod push;
mod statuses;
