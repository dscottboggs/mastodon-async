/// Data structure for the MastodonClient::add_filter method
pub use self::filter::AddFilterRequest;
/// Data structure for the MastodonClient::add_push_subscription method
pub use self::push::{AddPushRequest, Keys, UpdatePushRequest};
/// Data structure for the MastodonClient::add_report method
pub use self::report::AddReportRequest;
/// Data structure for the MastodonClient::statuses method
pub use self::statuses::StatusesRequest;

/// Requests specific to the admin API.
pub mod admin;
mod filter;
mod push;
mod report;
mod statuses;
