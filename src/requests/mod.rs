/// Data structure for the MastodonClient::add_filter method
pub use self::filter::AddFilterRequest;
/// Data structure for the MastodonClient::add_push_subscription method
pub use self::push::{AddPushSubscriptionRequest, PushRequestKeys, UpdatePushSubscriptionRequest};
/// Data structure for the MastodonClient::add_report method
pub use self::report::AddReportRequest;
/// Data structure for the MastodonClient::statuses method
pub use self::statuses::StatusesRequest;
/// Data structures for MastodonClient timeline methods
pub use self::timelines::{
    HashtagTimelineRequest, HomeTimelineRequest, ListTimelineRequest, PublicTimelineRequest,
};

/// Requests specific to the admin API.
pub mod admin;

/// Data structures for v1 and v2 filter methods.
pub mod filter;

mod push;
mod report;
mod statuses;
mod timelines;
