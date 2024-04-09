use std::time::Duration;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    filter::{self, FilterContext},
    helpers::serde_opt_duration_as_seconds,
};

#[derive(Builder, Debug, Deserialize, Serialize, Clone)]
#[builder(derive(Debug), build_fn(error = "crate::Error"))]
/// Form for creating a Filter.
///
/// See also [the API reference](https://docs.joinmastodon.org/methods/filters/#create)
pub struct Filter {
    /// The name of the filter group.
    title: String,
    /// Where the filter should be applied. Specify at least one.
    context: Vec<FilterContext>,
    /// The policy to be applied when the filter is matched.
    filter_action: filter::Action,
    /// How long from now should the filter expire?
    #[serde(
        with = "serde_opt_duration_as_seconds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    expires_in: Option<Duration>,
    /// A list of keywords to be added to the newly-created filter
    keywords_attributes: Vec<Keyword>,
}

#[derive(Debug, Deserialize, Serialize, Builder, Clone)]
#[builder(derive(Debug), build_fn(error = "crate::Error"))]
pub struct Keyword {
    /// A keyword to be added to the newly-created filter group
    keyword: String,
    /// Whether the keyword should consider word boundaries.
    whole_word: bool,
}
