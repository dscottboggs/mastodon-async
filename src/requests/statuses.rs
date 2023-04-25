use mastodon_async_derive::request_builder;
use mastodon_async_entities::StatusId;

/// Input to https://docs.joinmastodon.org/methods/accounts/#statuses
#[request_builder]
pub struct StatusesRequest {
    /// Return results older than this ID.
    pub max_id: Option<StatusId>,
    /// Return results newer than this ID.
    pub since_id: Option<StatusId>,
    /// Return results immediately newer than this ID.
    pub min_id: Option<StatusId>,
    /// Maximum number of results to return.
    pub limit: Option<i32>,
    /// Filter out statuses without attachments.
    pub only_media: Option<bool>,
    /// Filter out statuses in reply to a different account.
    pub exclude_replies: Option<bool>,
    /// Filter out boosts.
    pub exclude_reblogs: Option<bool>,
    /// Filter for pinned statuses only.
    pub pinned: Option<bool>,
    /// Filter for statuses using a specific hashtag.
    pub tagged: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn qs_test<F>(configure: F, expected_qs: &str)
    where
        F: FnOnce(&mut StatusesRequestBuilder),
    {
        let mut builder = StatusesRequest::builder();
        configure(&mut builder);
        let request = builder.build();
        let actual_qs =
            serde_urlencoded::to_string(request).expect("Failed to encode query string");
        assert_eq!(&actual_qs, expected_qs);
    }

    #[test]
    fn test_only_media() {
        qs_test(
            |request| {
                request.only_media(true);
            },
            "only_media=true",
        );
    }

    #[test]
    fn test_exclude_replies() {
        qs_test(
            |request| {
                request.exclude_replies(true);
            },
            "exclude_replies=true",
        );
    }

    #[test]
    fn test_pinned() {
        qs_test(
            |request| {
                request.pinned(true);
            },
            "pinned=true",
        );
    }

    #[test]
    fn test_max_id() {
        qs_test(
            |request| {
                request.max_id(StatusId::new("foo"));
            },
            "max_id=foo",
        );
    }

    #[test]
    fn test_limit() {
        qs_test(
            |request| {
                request.limit(42);
            },
            "limit=42",
        );
    }

    #[test]
    fn test_combination() {
        qs_test(
            |request| {
                request
                    .exclude_reblogs(true)
                    .exclude_replies(true)
                    .since_id(StatusId::new("foo"))
                    .limit(42);
            },
            "since_id=foo&limit=42&exclude_replies=true&exclude_reblogs=true",
        );
    }
}
