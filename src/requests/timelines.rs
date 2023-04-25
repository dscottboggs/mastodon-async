use mastodon_async_derive::request_builder;
use mastodon_async_entities::StatusId;

/// Input to https://docs.joinmastodon.org/methods/timelines/#public
#[request_builder]
pub struct PublicTimelineRequest {
    /// Show only local statuses.
    pub local: Option<bool>,
    /// Show only remote statuses.
    pub remote: Option<bool>,
    /// Show only statuses with media attached.
    pub only_media: Option<bool>,
    /// Return results older than this ID.
    pub max_id: Option<StatusId>,
    /// Return results newer than this ID.
    pub since_id: Option<StatusId>,
    /// Return results immediately newer than this ID.
    pub min_id: Option<StatusId>,
    /// Maximum number of results to return.
    pub limit: Option<i32>,
}

/// Input to https://docs.joinmastodon.org/methods/timelines/#tag
#[request_builder]
pub struct HashtagTimelineRequest {
    /// Return statuses that contain any of these additional tags.
    pub any: Option<Vec<String>>,
    /// Return statuses that contain all of these additional tags.
    pub all: Option<Vec<String>>,
    /// Return statuses that contain none of these additional tags.
    pub none: Option<Vec<String>>,
    /// Show only local statuses.
    pub local: Option<bool>,
    /// Show only remote statuses.
    pub remote: Option<bool>,
    /// Show only statuses with media attached.
    pub only_media: Option<bool>,
    /// Return results older than this ID.
    pub max_id: Option<StatusId>,
    /// Return results newer than this ID.
    pub since_id: Option<StatusId>,
    /// Return results immediately newer than this ID.
    pub min_id: Option<StatusId>,
    /// Maximum number of results to return.
    pub limit: Option<i32>,
}

/// Input to https://docs.joinmastodon.org/methods/timelines/#home
#[request_builder]
pub struct HomeTimelineRequest {
    /// Return results older than this ID.
    pub max_id: Option<StatusId>,
    /// Return results newer than this ID.
    pub since_id: Option<StatusId>,
    /// Return results immediately newer than this ID.
    pub min_id: Option<StatusId>,
    /// Maximum number of results to return.
    pub limit: Option<i32>,
}

/// Input to https://docs.joinmastodon.org/methods/timelines/#list
#[request_builder]
pub struct ListTimelineRequest {
    /// Return results older than this ID.
    pub max_id: Option<StatusId>,
    /// Return results newer than this ID.
    pub since_id: Option<StatusId>,
    /// Return results immediately newer than this ID.
    pub min_id: Option<StatusId>,
    /// Maximum number of results to return.
    pub limit: Option<i32>,
}
