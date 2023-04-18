use crate::entities::filter::{Action, FilterContext};
use derive_builder::Builder;
use mastodon_async_derive::MandatoryParamBuilder;
use serde_with::{serde_as, skip_serializing_none, DurationSeconds};
use time::Duration;

/// Form used to create a v2 filter.
///
/// - https://docs.joinmastodon.org/methods/filters/#create
///
/// // Example
///
/// ```
/// use mastodon_async::{entities::filter::FilterContext, requests::filter::v1::AddFilterRequest};
/// let request = AddFilterRequest::builder("foo", vec![FilterContext::Home]).build();
/// ```
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Builder, MandatoryParamBuilder)]
#[builder(
    derive(Debug, PartialEq),
    custom_constructor,
    build_fn(private, name = "try_build"),
    setter(into, strip_option)
)]
pub struct AddFilterRequest {
    /// The name of the filter group.
    #[builder(private)]
    pub title: String,
    /// Where the filter should be applied. Specify at least one context.
    #[builder(private)]
    pub context: Vec<FilterContext>,
    /// The policy to be applied when the filter is matched.
    #[builder(default)]
    pub filter_action: Option<Action>,
    /// How many seconds from now should the filter expire? Omit to create filters that do not expire.
    #[builder(default)]
    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    pub expires_in: Option<Duration>,
    /// Filter keywords with attributes.
    #[builder(default)]
    pub keywords_attributes: Option<Vec<AddFilterKeyword>>,
}

/// A keyword with attributes, used when creating a filter.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Builder, MandatoryParamBuilder)]
#[builder(
    derive(Debug, PartialEq),
    custom_constructor,
    build_fn(private, name = "try_build"),
    setter(into, strip_option)
)]
pub struct AddFilterKeyword {
    /// A keyword to be added to the newly-created filter group.
    pub keyword: String,
    /// Whether the keyword should consider word boundaries.
    pub whole_word: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use time::Duration;

    #[test]
    fn test_serialize_request() {
        let request = AddFilterRequest::builder("foo fighters", vec![FilterContext::Home])
            .keywords_attributes(vec![AddFilterKeyword::builder("foo", true).build()])
            .expires_in(Duration::seconds(300))
            .build();
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(
            ser,
            r#"{"title":"foo","context":["home"],"expires_in":300,"keywords_attributes":[{"keyword":"foo","whole_word":true}]}"#
        )
    }
}

/// Create client-side filters used by Mastodon versions before 4 and some non-Mastodon software.
pub mod v1 {
    use crate::entities::filter::FilterContext;
    use derive_builder::Builder;
    use mastodon_async_derive::MandatoryParamBuilder;
    use serde_with::{serde_as, skip_serializing_none, DurationSeconds};
    use time::Duration;

    /// Form used to create or update a v1 filter.
    ///
    /// - https://docs.joinmastodon.org/methods/filters/#create-v1
    /// - https://docs.joinmastodon.org/methods/filters/#update-v1
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::{entities::filter::FilterContext, requests::filter::v1::AddFilterRequest};
    /// let request = AddFilterRequest::builder("foo", vec![FilterContext::Home]).build();
    /// ```
    #[serde_as]
    #[skip_serializing_none]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Builder, MandatoryParamBuilder)]
    #[builder(
        derive(Debug, PartialEq),
        custom_constructor,
        build_fn(private, name = "try_build"),
        setter(into, strip_option)
    )]
    pub struct AddFilterRequest {
        /// The text to be filtered.
        #[builder(private)]
        pub phrase: String,
        /// Where the filter should be applied.
        #[builder(private)]
        pub context: Vec<FilterContext>,
        /// Should the server irreversibly drop matching entities from home and notifications?
        #[builder(default)]
        pub irreversible: Option<bool>,
        /// Should the filter consider word boundaries for this keyword?
        #[builder(default)]
        pub whole_word: Option<bool>,
        /// Number of seconds from now that the filter should expire.
        #[builder(default)]
        #[serde_as(as = "Option<DurationSeconds<i64>>")]
        pub expires_in: Option<Duration>,
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use serde_json;
        use time::Duration;

        #[test]
        fn test_serialize_request() {
            let request = AddFilterRequest::builder("foo", vec![FilterContext::Home])
                .expires_in(Duration::seconds(300))
                .build();
            let ser = serde_json::to_string(&request).expect("Couldn't serialize");
            assert_eq!(
                ser,
                r#"{"phrase":"foo","context":["home"],"expires_in":300}"#
            )
        }
    }
}
