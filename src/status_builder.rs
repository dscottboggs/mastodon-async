use std::str::FromStr;

use isolang::Language;
use serde::{Deserialize, Serialize};

/// A builder pattern struct for constructing a status.
///
/// // Example
///
/// ```
/// use mastodon_async::{Language, StatusBuilder};
///
/// let status = StatusBuilder::new()
///     .status("a status")
///     .sensitive(true)
///     .spoiler_text("a CW")
///     .language(Language::Eng)
///     .build().unwrap();
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StatusBuilder {
    status: Option<String>,
    in_reply_to_id: Option<String>,
    media_ids: Option<Vec<String>>,
    sensitive: Option<bool>,
    spoiler_text: Option<String>,
    content_type: Option<String>,
    visibility: Option<Visibility>,
    language: Option<Language>,
}

impl StatusBuilder {
    /// Create a StatusBuilder object
    ///
    /// // Example
    ///
    /// ```rust,no_run
    /// use mastodon_async::{status_builder::Visibility, prelude::*};
    ///
    /// let data = Data::default();
    /// let client = Mastodon::from(data);
    /// let status = StatusBuilder::new()
    ///     .status("a status")
    ///     .visibility(Visibility::Public)
    ///     .build()
    ///     .unwrap();
    ///
    /// tokio_test::block_on(async {
    ///     client.new_status(status).await.unwrap();
    /// });
    /// ```
    pub fn new() -> StatusBuilder {
        StatusBuilder::default()
    }

    /// Set the text for the post
    ///
    /// // Example
    ///
    /// ```rust
    /// use mastodon_async::prelude::*;
    /// let status = StatusBuilder::new().status("awoooooo").build().unwrap();
    /// ```
    pub fn status<I: Into<String>>(&mut self, status: I) -> &mut Self {
        self.status = Some(status.into());
        self
    }

    /// Set the in_reply_to_id for the post
    ///
    /// // Example
    ///
    /// ```rust
    /// use mastodon_async::prelude::*;
    /// let status = StatusBuilder::new()
    ///     .status("awooooo")
    ///     .in_reply_to("12345")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn in_reply_to<I: Into<String>>(&mut self, id: I) -> &mut Self {
        self.in_reply_to_id = Some(id.into());
        self
    }

    /// Set the media_ids for the post
    ///
    /// // Example
    ///
    /// ```rust
    /// use mastodon_async::prelude::*;
    /// let status = StatusBuilder::new().media_ids(&["foo", "bar"]).build().unwrap();
    /// ```
    pub fn media_ids<S: std::fmt::Display, I: IntoIterator<Item = S>>(
        &mut self,
        ids: I,
    ) -> &mut Self {
        self.media_ids = Some(ids.into_iter().map(|s| s.to_string()).collect::<Vec<_>>());
        self
    }

    /// Set the sensitive attribute for the post
    ///
    /// // Example
    ///
    /// ```rust
    /// use mastodon_async::prelude::*;
    /// let status = StatusBuilder::new()
    ///     .media_ids(&["foo", "bar"])
    ///     .sensitive(true)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn spoiler_text<I: Into<String>>(&mut self, spoiler_text: I) -> &mut Self {
        self.spoiler_text = Some(spoiler_text.into());
        self
    }

    /// Set the content type of the post
    ///
    /// This is a Pleroma and Glitch-soc extension of the API.
    ///
    /// // Possible values
    /// - `text/plain`
    /// - `text/html`
    /// - `text/markdown`
    /// - `text/bbcode` (Pleroma only)
    ///
    /// The set of supported content types may vary by server.
    ///
    /// // Example
    ///
    /// ```rust
    /// use mastodon_async::prelude::*;
    /// let status = StatusBuilder::new()
    ///     .status("<b>thicc</b>")
    ///     .content_type("text/html")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn content_type<I: Into<String>>(&mut self, content_type: I) -> &mut Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// Set the visibility for the post
    ///
    /// // Example
    ///
    /// ```rust
    /// use mastodon_async::{prelude::*, status_builder::Visibility};
    /// let status = StatusBuilder::new()
    ///     .status("awooooooo")
    ///     .visibility(Visibility::Public)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn visibility(&mut self, visibility: Visibility) -> &mut Self {
        self.visibility = Some(visibility);
        self
    }

    /// Set the language for the post
    ///
    /// // Example
    ///
    /// ```rust
    /// use mastodon_async::{Language, prelude::*};
    /// let status = StatusBuilder::new()
    ///     .status("awoo!!!!")
    ///     .language(Language::Eng)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn language(&mut self, language: Language) -> &mut Self {
        self.language = Some(language);
        self
    }

    /// Set the status as "sensitive".
    /// ```
    /// use mastodon_async::StatusBuilder;
    ///
    /// let status = StatusBuilder::new()
    ///     .status("a sensitive matter")
    ///     .sensitive(true)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn sensitive(&mut self, flag: bool) -> &mut Self {
        self.sensitive = Some(flag);
        self
    }

    /// Constructs a NewStatus
    ///
    /// // Example
    ///
    /// ```rust
    /// use mastodon_async::prelude::*;
    /// let status = StatusBuilder::new().status("awoo!").build().unwrap();
    /// ```
    pub fn build(&self) -> Result<NewStatus, crate::Error> {
        if self.status.is_none() && self.media_ids.is_none() {
            return Err(crate::Error::Other(
                "status text or media ids are required in order to post a status".to_string(),
            ));
        }
        Ok(NewStatus {
            status: self.status.clone(),
            in_reply_to_id: self.in_reply_to_id.clone(),
            media_ids: self.media_ids.clone(),
            sensitive: self.sensitive,
            spoiler_text: self.spoiler_text.clone(),
            visibility: self.visibility,
            language: self.language,
            content_type: self.content_type.clone(),
        })
    }
}

/// Represents a post that can be sent to the POST /api/v1/status endpoint
#[derive(Debug, Default, Clone, Serialize, PartialEq, Eq)]
pub struct NewStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_reply_to_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spoiler_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<Visibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<String>,
}

/// The visibility of a status.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    /// A Direct message to a user
    Direct,
    /// Only available to followers
    Private,
    /// Not shown in public timelines
    Unlisted,
    /// Posted to public timelines
    Public,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Public
    }
}

impl FromStr for Visibility {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "direct" => Ok(Visibility::Direct),
            "private" => Ok(Visibility::Private),
            "unlisted" => Ok(Visibility::Unlisted),
            "public" => Ok(Visibility::Public),
            invalid => Err(format!("unrecognized visibility '{invalid}'").into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use isolang::Language;
    use serde_json;

    #[test]
    fn test_new() {
        let s = StatusBuilder::new()
            .status("a status")
            .build()
            .expect("Couldn't build status");
        let expected = NewStatus {
            status: Some("a status".to_string()),
            in_reply_to_id: None,
            media_ids: None,
            sensitive: None,
            spoiler_text: None,
            visibility: None,
            language: None,
            content_type: None,
        };
        assert_eq!(s, expected);
    }

    #[test]
    fn test_default_visibility() {
        let v: Visibility = Default::default();
        assert_eq!(v, Visibility::Public);
    }

    #[test]
    fn test_serialize_visibility() {
        assert_eq!(
            serde_json::to_string(&Visibility::Direct).expect("couldn't serialize visibility"),
            "\"direct\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&Visibility::Private).expect("couldn't serialize visibility"),
            "\"private\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&Visibility::Unlisted).expect("couldn't serialize visibility"),
            "\"unlisted\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&Visibility::Public).expect("couldn't serialize visibility"),
            "\"public\"".to_string()
        );
    }

    #[test]
    fn test_serialize_status() {
        let status = StatusBuilder::new()
            .status("a status")
            .build()
            .expect("Couldn't build status");
        assert_eq!(
            serde_json::to_string(&status).expect("Couldn't serialize status"),
            "{\"status\":\"a status\"}".to_string()
        );

        let status = StatusBuilder::new()
            .status("a status")
            .language(Language::Eng)
            .build()
            .expect("Couldn't build status");
        assert_eq!(
            serde_json::to_string(&status).expect("Couldn't serialize status"),
            "{\"status\":\"a status\",\"language\":\"eng\"}"
        );
    }
}
