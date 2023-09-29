/// Data structure for the MastodonClient::add_filter method
pub use self::filter::AddFilterRequest;
/// Data structure for the MastodonClient::add_push_subscription method
pub use self::push::{AddPushRequest, Keys, UpdatePushRequest};

mod filter;
mod push;
