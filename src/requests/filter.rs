use crate::entities::filter::FilterContext;
use mastodon_async_entities::filter::Action;
use std::time::Duration;
use time::{serde::iso8601, OffsetDateTime};

/// Form used to create a filter
///
/// // Example
///
/// ```
/// use mastodon_async::{entities::filter::FilterContext, requests::AddFilterRequest};
/// let request = AddFilterRequest::new("foo", FilterContext::Home);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddFilterRequest {
    phrase: String,
    context: FilterContext,
    irreversible: Option<bool>,
    whole_word: Option<bool>,
    #[serde(serialize_with = "serialize_duration::ser")]
    expires_in: Option<Duration>,
}

impl AddFilterRequest {
    /// Create a new AddFilterRequest
    pub fn new(phrase: &str, context: FilterContext) -> AddFilterRequest {
        AddFilterRequest {
            phrase: phrase.to_string(),
            context,
            irreversible: None,
            whole_word: None,
            expires_in: None,
        }
    }

    /// Set `irreversible` to `true`
    pub fn irreversible(&mut self) -> &mut Self {
        self.irreversible = Some(true);
        self
    }

    /// Set `whole_word` to `true`
    pub fn whole_word(&mut self) -> &mut Self {
        self.whole_word = Some(true);
        self
    }

    /// Set `expires_in` to a duration
    pub fn expires_in(&mut self, d: Duration) -> &mut Self {
        self.expires_in = Some(d);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AddFilterV2Keyword {
    /// The phrase to be matched against.
    keyword: String,
    /// Should the filter consider word boundaries? See [implementation guidelines
    /// for filters](https://docs.joinmastodon.org/api/guidelines/#filters).
    whole_word: bool,
}

/// Form used to create a v2 filter
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddFilterV2Request {
    /// A title given by the user to name the filter.
    title: String,
    /// The contexts in which the filter should be applied.
    context: Vec<FilterContext>,
    /// When the filter should no longer be applied.
    #[serde(with = "iso8601::option")]
    expires_at: Option<OffsetDateTime>,
    /// The action to be taken when a status matches this filter.
    filter_action: Action,
    /// The keywords grouped under this filter.
    keywords_attributes: Vec<AddFilterV2Keyword>,
}

#[allow(dead_code)]
impl AddFilterV2Request {
    /// Create a new AddFilterV2Request
    pub fn new(title: &str, context: Vec<FilterContext>, action: Action) -> AddFilterV2Request {
        Self {
            title: title.to_string(),
            context,
            expires_at: None,
            filter_action: action,
            keywords_attributes: vec![],
        }
    }

    /// Set `expires_at` for the filter
    pub fn expires_at(&mut self, d: OffsetDateTime) -> &mut Self {
        self.expires_at = Some(d);
        self
    }

    /// Adds a keyword to the filter
    pub fn with_keyword(&mut self, keyword: &str, whole_word: bool) -> &mut Self {
        self.keywords_attributes.push(AddFilterV2Keyword {
            keyword: keyword.to_string(),
            whole_word,
        });
        self
    }
}

mod serialize_duration {
    use serde::ser::Serializer;
    use std::time::Duration;

    pub(crate) fn ser<S>(duration: &Option<Duration>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(d) = duration {
            let sec = d.as_secs();
            s.serialize_u64(sec)
        } else {
            s.serialize_none()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::time::Duration;

    #[test]
    fn test_new() {
        let request = AddFilterRequest::new("foo", FilterContext::Home);
        assert_eq!(
            request,
            AddFilterRequest {
                phrase: "foo".to_string(),
                context: FilterContext::Home,
                irreversible: None,
                whole_word: None,
                expires_in: None,
            }
        )
    }

    #[test]
    fn test_irreversible() {
        let mut request = AddFilterRequest::new("foo", FilterContext::Home);
        request.irreversible();
        assert_eq!(
            request,
            AddFilterRequest {
                phrase: "foo".to_string(),
                context: FilterContext::Home,
                irreversible: Some(true),
                whole_word: None,
                expires_in: None,
            }
        )
    }

    #[test]
    fn test_whole_word() {
        let mut request = AddFilterRequest::new("foo", FilterContext::Home);
        request.whole_word();
        assert_eq!(
            request,
            AddFilterRequest {
                phrase: "foo".to_string(),
                context: FilterContext::Home,
                irreversible: None,
                whole_word: Some(true),
                expires_in: None,
            }
        )
    }

    #[test]
    fn test_expires_in() {
        let mut request = AddFilterRequest::new("foo", FilterContext::Home);
        request.expires_in(Duration::from_secs(300));
        assert_eq!(
            request,
            AddFilterRequest {
                phrase: "foo".to_string(),
                context: FilterContext::Home,
                irreversible: None,
                whole_word: None,
                expires_in: Some(Duration::from_secs(300)),
            }
        )
    }

    #[test]
    fn test_serialize_request() {
        let mut request = AddFilterRequest::new("foo", FilterContext::Home);
        request.expires_in(Duration::from_secs(300));
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(
            ser,
            r#"{"phrase":"foo","context":"home","irreversible":null,"whole_word":null,"expires_in":300}"#
        )
    }
}
