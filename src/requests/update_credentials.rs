use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use crate::{
    entities::account::{Credentials, MetadataField, UpdateSource},
    errors::Result,
};
use mastodon_async_entities::visibility::Visibility;

/// Builder to pass to the Mastodon::update_credentials method
///
/// // Example
///
/// ```no_run
/// use mastodon_async::{prelude::*, entities::visibility::Visibility, UpdateCredsRequest};
///
/// let data = Data::default();
/// let client = Mastodon::from(data);
/// let mut builder = UpdateCredsRequest::new();
///
/// builder.privacy(Visibility::Unlisted);
///
/// tokio_test::block_on(async {
///     let result = client.update_credentials(&mut builder).await.unwrap();
/// });
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct UpdateCredsRequest {
    display_name: Option<String>,
    note: Option<String>,
    avatar: Option<PathBuf>,
    header: Option<PathBuf>,
    field_attributes: Vec<MetadataField>,

    // UpdateSource fields
    privacy: Option<Visibility>,
    sensitive: Option<bool>,
}

impl UpdateCredsRequest {
    /// Create a new UpdateCredsRequest
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::UpdateCredsRequest;
    ///
    /// let mut builder = UpdateCredsRequest::new();
    /// ```
    pub fn new() -> UpdateCredsRequest {
        Default::default()
    }

    /// Set the new display_name value
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::UpdateCredsRequest;
    ///
    /// let mut builder = UpdateCredsRequest::new();
    ///
    /// builder.display_name("my new display name");
    /// ```
    pub fn display_name<D: Display>(&mut self, name: D) -> &mut Self {
        self.display_name = Some(name.to_string());
        self
    }

    /// Set the new note value
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::UpdateCredsRequest;
    ///
    /// let mut builder = UpdateCredsRequest::new();
    ///
    /// builder.note("my new note");
    /// ```
    pub fn note<D: Display>(&mut self, note: D) -> &mut Self {
        self.note = Some(note.to_string());
        self
    }

    /// Set the new avatar value
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::UpdateCredsRequest;
    ///
    /// let mut builder = UpdateCredsRequest::new();
    ///
    /// builder.avatar("/path/to/my/new/avatar");
    /// ```
    pub fn avatar<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        let path = path.as_ref();
        let path = path.to_path_buf();
        self.avatar = Some(path);
        self
    }

    /// Set the new header value
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::UpdateCredsRequest;
    ///
    /// let mut builder = UpdateCredsRequest::new();
    ///
    /// builder.header("/path/to/my/new/header");
    /// ```
    pub fn header<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        let path = path.as_ref();
        let path = path.to_path_buf();
        self.header = Some(path);
        self
    }

    /// Set the new privacy value
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::{entities::visibility::Visibility, UpdateCredsRequest};
    ///
    /// let mut builder = UpdateCredsRequest::new();
    ///
    /// builder.privacy(Visibility::Public);
    /// ```
    pub fn privacy(&mut self, privacy: Visibility) -> &mut Self {
        self.privacy = Some(privacy);
        self
    }

    /// Set the new sensitive value
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::UpdateCredsRequest;
    ///
    /// let mut builder = UpdateCredsRequest::new();
    ///
    /// builder.sensitive(true);
    /// ```
    pub fn sensitive(&mut self, sensitive: bool) -> &mut Self {
        self.sensitive = Some(sensitive);
        self
    }

    /// Add a metadata field
    ///
    /// // Example
    ///
    /// ```
    /// use mastodon_async::UpdateCredsRequest;
    ///
    /// let mut builder = UpdateCredsRequest::new();
    ///
    /// builder.field_attribute("some key", "some value");
    /// ```
    pub fn field_attribute(&mut self, name: &str, value: &str) -> &mut Self {
        self.field_attributes.push(MetadataField::new(name, value));
        self
    }

    pub(crate) fn build(&mut self) -> Result<Credentials> {
        Ok(Credentials {
            display_name: self.display_name.clone(),
            note: self.note.clone(),
            avatar: self.avatar.clone(),
            header: self.avatar.clone(),
            source: Some(UpdateSource {
                privacy: self.privacy,
                sensitive: self.sensitive,
            }),
            fields_attributes: self.field_attributes.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::account::{Credentials, MetadataField, UpdateSource};
    use mastodon_async_entities::visibility::Visibility;

    #[test]
    fn test_update_creds_request_new() {
        let builder = UpdateCredsRequest::new();
        assert_eq!(
            builder,
            UpdateCredsRequest {
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_display_name() {
        let mut builder = UpdateCredsRequest::new();
        builder.display_name("foo");
        assert_eq!(
            builder,
            UpdateCredsRequest {
                display_name: Some("foo".into()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_note() {
        let mut builder = UpdateCredsRequest::new();
        builder.note("foo");
        assert_eq!(
            builder,
            UpdateCredsRequest {
                note: Some("foo".into()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_avatar() {
        let mut builder = UpdateCredsRequest::new();
        builder.avatar("/path/to/avatar.png");
        assert_eq!(
            builder,
            UpdateCredsRequest {
                avatar: Some(Path::new("/path/to/avatar.png").to_path_buf()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_header() {
        let mut builder = UpdateCredsRequest::new();
        builder.header("/path/to/header.png");
        assert_eq!(
            builder,
            UpdateCredsRequest {
                header: Some(Path::new("/path/to/header.png").to_path_buf()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_privacy() {
        let mut builder = UpdateCredsRequest::new();
        builder.privacy(Visibility::Public);
        assert_eq!(
            builder,
            UpdateCredsRequest {
                privacy: Some(Visibility::Public),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_sensitive() {
        let mut builder = UpdateCredsRequest::new();
        builder.sensitive(true);
        assert_eq!(
            builder,
            UpdateCredsRequest {
                sensitive: Some(true),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_field_attribute() {
        let mut builder = UpdateCredsRequest::new();
        builder.field_attribute("foo", "bar");
        assert_eq!(
            builder,
            UpdateCredsRequest {
                field_attributes: vec![MetadataField::new("foo", "bar")],
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_creds_request_build() {
        let mut builder = UpdateCredsRequest::new();
        builder.display_name("test").note("a note");
        let creds = builder.build().expect("Couldn't build Credentials");
        assert_eq!(
            creds,
            Credentials {
                display_name: Some("test".into()),
                note: Some("a note".into()),
                source: Some(UpdateSource {
                    ..Default::default()
                }),
                ..Default::default()
            }
        );
    }
}
