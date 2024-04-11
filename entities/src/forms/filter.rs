use time::Duration;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{helpers::serde_opt_duration_as_seconds, prelude::*};

#[derive(Builder, Debug, Default, Deserialize, Serialize, Clone)]
#[builder(derive(Debug), build_fn(error = "crate::Error"))]
/// Form for creating a Filter.
///
/// ```
/// use mastodon_async_entities::prelude::*;
/// use time::ext::NumericalDuration;
///
/// let filter = forms::filter::Add::builder("test filter")
///     .add_context(filter::Context::Home)
///     .filter_action(filter::Action::Hide)
///     .expires_in(60.seconds())
///     .keyword(forms::filter::add::Keyword::whole_word("test"))
///     .keyword(forms::filter::add::Keyword::substring("substring you really don't want to see"))
///     .build()
///     .unwrap();
/// assert_eq!(serde_json::to_string_pretty(&filter).unwrap(), r#"{
///   "title": "test filter",
///   "context": [
///     "home"
///   ],
///   "filter_action": "hide",
///   "expires_in": 60,
///   "keywords_attributes": [
///     {
///       "keyword": "test",
///       "whole_word": true
///     },
///     {
///       "keyword": "substring you really don't want to see",
///       "whole_word": false
///     }
///   ]
/// }"#);
/// ```
///
/// See also [the API reference](https://docs.joinmastodon.org/methods/filters/#create)
pub struct Add {
    /// The name of the filter group.
    #[builder(setter(custom), default)]
    title: String,
    /// Where the filter should be applied. Specify at least one.
    #[serde(serialize_with = "add::disallow_empty_context")]
    #[builder(default, setter(into, strip_option))]
    context: Vec<filter::Context>,
    /// The policy to be applied when the filter is matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    filter_action: Option<filter::Action>,
    /// How long from now should the filter expire?
    #[serde(
        with = "serde_opt_duration_as_seconds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    #[builder(default, setter(into, strip_option))]
    expires_in: Option<Duration>,
    /// A list of keywords to be added to the newly-created filter
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default, setter(into, strip_option))]
    keywords_attributes: Vec<add::Keyword>,
}

impl Add {
    pub fn builder(title: impl Into<String>) -> AddBuilder {
        AddBuilder {
            title: Some(title.into()),
            ..Default::default()
        }
    }
}

impl AddBuilder {
    pub fn add_context(&mut self, context: filter::Context) -> &mut Self {
        self.context
            .get_or_insert_with(Default::default)
            .push(context);
        self
    }
    pub fn keyword(&mut self, keyword: add::Keyword) -> &mut Self {
        self.keywords_attributes
            .get_or_insert_with(Default::default)
            .push(keyword);
        self
    }
}

pub mod add {
    use derive_builder::Builder;
    use serde::{ser, Deserialize, Serialize, Serializer};

    use crate::prelude::*;

    #[derive(Debug, Deserialize, Serialize, Builder, Clone)]
    pub struct Keyword {
        /// A keyword to be added to the newly-created filter group
        keyword: String,
        /// Whether the keyword should consider word boundaries.
        whole_word: bool,
    }

    impl Keyword {
        pub fn new(keyword: String, whole_word: bool) -> Self {
            Self {
                keyword,
                whole_word,
            }
        }
        /// A filter keyword which should match only if it is seen as a word or
        /// phrase among other phrases.
        pub fn whole_word(keyword: impl Into<String>) -> Self {
            Self {
                keyword: keyword.into(),
                whole_word: true,
            }
        }
        /// A filter keyword which should match even if it's seen as part of
        /// another word.
        pub fn substring(keyword: impl Into<String>) -> Self {
            Self {
                keyword: keyword.into(),
                whole_word: false,
            }
        }
    }

    pub(super) fn disallow_empty_context<S>(
        context: impl AsRef<[filter::Context]>,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let context = context.as_ref();
        if context.is_empty() {
            Err(ser::Error::custom("filter context cannot be empty"))
        } else {
            context.serialize(s)
        }
    }
}

#[derive(Builder, Debug, Deserialize, Serialize, Clone)]
#[builder(derive(Debug), build_fn(error = "crate::Error"))]
/// Form for updating a Filter.
///
/// ```
/// use mastodon_async_entities::prelude::*;
/// use time::ext::NumericalDuration;
///
/// let keyword = forms::filter::update::Keyword::builder()
///     .keyword("test")
///     .whole_word(false)
///     .id("this won't work")
///     .destroy(true)
///     .build()
///     .unwrap();
/// let filter = forms::filter::Update::builder()
///     // note that the ID isn't here: it's in the URL, not passed as a part
///     // of the form
///     .title("test filter")
///     .add_context(filter::Context::Home)
///     .filter_action(filter::Action::Hide)
///     .expires_in(60.seconds())
///     .keyword(keyword)
///     .build()
///     .unwrap();
/// assert_eq!(serde_json::to_string_pretty(&filter).unwrap(), r#"{
///   "title": "test filter",
///   "context": [
///     "home"
///   ],
///   "filter_action": "hide",
///   "expires_in": 60,
///   "keywords_attributes": [
///     {
///       "keyword": "test",
///       "whole_word": false,
///       "id": "this won't work",
///       "destroy": true
///     }
///   ]
/// }"#);
/// ```
///
/// See also [the API reference](https://docs.joinmastodon.org/methods/filters/#update)
pub struct Update {
    /// The name of the filter group.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    title: Option<String>,
    /// Where the filter should be applied. Specify at least one.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default, setter(into))]
    context: Vec<filter::Context>,
    /// The policy to be applied when the filter is matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    filter_action: Option<filter::Action>,
    /// How long from now should the filter expire?
    #[serde(
        with = "serde_opt_duration_as_seconds",
        skip_serializing_if = "Option::is_none",
        default
    )]
    #[builder(default, setter(strip_option, into))]
    expires_in: Option<Duration>,
    /// A list of keywords to be added to the newly-created filter
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default, setter(into))]
    keywords_attributes: Vec<update::Keyword>,
}

impl Update {
    pub fn builder() -> UpdateBuilder {
        Default::default()
    }
}

impl UpdateBuilder {
    pub fn add_context(&mut self, context: filter::Context) -> &mut Self {
        self.context
            .get_or_insert_with(Default::default)
            .push(context);
        self
    }
    /// Add a `update::Keyword` to the list of keywords. May be specified multiple times
    pub fn keyword(&mut self, keyword: update::Keyword) -> &mut Self {
        self.keywords_attributes
            .get_or_insert_with(Default::default)
            .push(keyword);
        self
    }
}

pub mod update {
    use crate::helpers::is_false;
    use derive_builder::Builder;
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Deserialize, Serialize, Builder, Clone)]
    #[builder(derive(Debug), build_fn(error = "crate::Error"))]
    pub struct Keyword {
        /// A keyword to be added to the newly-created filter group
        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(default, setter(strip_option, into))]
        keyword: Option<String>,
        /// Whether the keyword should consider word boundaries.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(default, setter(strip_option, into))]
        whole_word: Option<bool>,
        /// Provide the ID of an existing keyword to modify it, instead of creating a new keyword.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(default, setter(strip_option, into))]
        id: Option<String>,
        /// If true, will remove the keyword with the given ID.
        #[serde(skip_serializing_if = "is_false")]
        #[builder(default)]
        destroy: bool,
    }

    impl Keyword {
        pub fn builder() -> KeywordBuilder {
            Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Status {
    status_id: StatusId,
}

impl Status {
    pub fn new(status_id: StatusId) -> Self {
        Self { status_id }
    }
}
