use std::{borrow::Cow, ops::Deref, path::Path, sync::Arc};

use crate::{
    entities::{account::Account, prelude::*, report::Report, status::Status, Empty},
    errors::{Error, Result},
    helpers::read_response::read_response,
    log_serde,
    polling_time::PollingTime,
    AddFilterRequest, AddPushRequest, Data, NewStatus, Page, StatusesRequest, UpdatePushRequest,
};
use futures::TryStream;
use log::{as_debug, as_serde, debug, error, trace};
use mastodon_async_entities::{account::CredentialsBuilder, attachment::ProcessedAttachment};
use reqwest::{multipart::Part, Client, RequestBuilder};
use url::Url;
use uuid::Uuid;

/// The Mastodon client is a smart pointer to this struct
#[derive(Debug)]
pub struct MastodonClient {
    pub(crate) client: Client,
    /// Raw data about your mastodon instance.
    pub data: Data,
}

/// Your mastodon application client, handles all requests to and from Mastodon.
#[derive(Debug, Clone)]
pub struct Mastodon(Arc<MastodonClient>);

// This ensures we don't accidentally make Mastodon not Send or Sync again
static_assertions::assert_impl_all!(Mastodon: Send, Sync);

/// A client for making unauthenticated requests to the public API.
#[derive(Clone, Debug)]
pub struct MastodonUnauthenticated {
    client: Client,
    /// Which Mastodon instance to contact
    pub base: Url,
}

impl From<Data> for Mastodon {
    /// Creates a mastodon instance from the data struct.
    fn from(data: Data) -> Mastodon {
        Mastodon::new(Client::new(), data)
    }
}
impl Mastodon {
    methods![get and get_with_call_id, post and post_with_call_id, delete and delete_with_call_id,];

    paged_routes! {
        (get) favourites: "favourites" => Status,
        (get) blocks: "blocks" => Account,
        (get) domain_blocks: "domain_blocks" => String,
        (get) instance_domain_blocks: "instance/domain_blocks" => DomainBlock,
        (get) follow_requests: "follow_requests" => Account,
        (get) get_home_timeline: "timelines/home" => Status,
        (get) get_emojis: "custom_emojis" => CustomEmoji,
        (get) mutes: "mutes" => Account,
        (get) notifications: "notifications" => Notification,
        (get) instance_peers: "instance/peers" => String,
        (get) instance_activity: "instance/activity" => Activity,
        (get) instance_rules: "instance/rules" => Rule,
        (get) reports: "reports" => Report,
        (get (q: &'a str, #[serde(skip_serializing_if = "Option::is_none")] limit: Option<u64>, following: bool,)) search_accounts: "accounts/search" => Account,
        (get) get_endorsements: "endorsements" => Account,
    }

    paged_routes_with_id! {
        (get) followers: "accounts/{}/followers" => Account,
        (get) following: "accounts/{}/following" => Account,
        (get) reblogged_by: "statuses/{}/reblogged_by" => Account,
        (get) favourited_by: "statuses/{}/favourited_by" => Account,
    }

    route! {
        (delete (domain: String,)) unblock_domain: "domain_blocks" => Empty,
        (get) instance: "instance" => Instance,
        (get) verify_credentials: "accounts/verify_credentials" => Account,
        (post (account_id: &str, status_ids: Vec<&str>, comment: String,)) report: "reports" => Report,
        (post (domain: String,)) block_domain: "domain_blocks" => Empty,
        (post (id: &str,)) authorize_follow_request: "accounts/follow_requests/authorize" => Empty,
        (post (id: &str,)) reject_follow_request: "accounts/follow_requests/reject" => Empty,
        (get  (local: bool,)) get_public_timeline: "timelines/public" => Vec<Status>,
        (post (uri: Cow<'static, str>,)) follows: "follows" => Account,
        (post) clear_notifications: "notifications/clear" => Empty,
        (post (id: &str,)) dismiss_notification: "notifications/dismiss" => Empty,
        (get) get_push_subscription: "push/subscription" => Subscription,
        (delete) delete_push_subscription: "push/subscription" => Empty,
        (get) get_filters: "filters" => Vec<Filter>,
        (get) get_follow_suggestions: "suggestions" => Vec<Account>,
    }

    route_v2! {
        (get (q: &'a str, resolve: bool,)) search: "search" => SearchResult,
        (post multipart with description (file: impl AsRef<Path>,)) media: "media" => Attachment,
        (post multipart with description (file: impl AsRef<Path>, thumbnail: impl AsRef<Path>,)) media_with_thumbnail: "media" => Attachment,
    }

    route_id! {
        (get) get_account[AccountId]: "accounts/{}" => Account,
        (post) follow[AccountId]: "accounts/{}/follow" => Relationship,
        (post) unfollow[AccountId]: "accounts/{}/unfollow" => Relationship,
        (post) block[AccountId]: "accounts/{}/block" => Relationship,
        (post) unblock[AccountId]: "accounts/{}/unblock" => Relationship,
        (get) mute[AccountId]: "accounts/{}/mute" => Relationship,
        (get) unmute[AccountId]: "accounts/{}/unmute" => Relationship,
        (get) get_notification[NotificationId]: "notifications/{}" => Notification,
        (get) get_status[StatusId]: "statuses/{}" => Status,
        (get) get_context[StatusId]: "statuses/{}/context" => Context,
        (get) get_card[StatusId]: "statuses/{}/card" => Card,
        (post) reblog[StatusId]: "statuses/{}/reblog" => Status,
        (post) unreblog[StatusId]: "statuses/{}/unreblog" => Status,
        (post) favourite[StatusId]: "statuses/{}/favourite" => Status,
        (post) unfavourite[StatusId]: "statuses/{}/unfavourite" => Status,
        (delete) delete_status[StatusId]: "statuses/{}" => Empty,
        (get) get_filter[FilterId]: "filters/{}" => Filter,
        (delete) delete_filter[FilterId]: "filters/{}" => Empty,
        (delete) delete_from_suggestions[AccountId]: "suggestions/{}" => Empty,
        (post) endorse_user[AccountId]: "accounts/{}/pin" => Relationship,
        (post) unendorse_user[AccountId]: "accounts/{}/unpin" => Relationship,
        (get) attachment[AttachmentId]: "media/{}" => Attachment,
    }

    streaming! {
        "returns events that are relevant to the authorized user, i.e. home timeline & notifications"
        stream_user@"user",
        "All public posts known to the server. Analogous to the federated timeline."
        stream_public@"public",
        "All public posts known to the server, filtered for media attachments. Analogous to the federated timeline with 'only media' enabled."
        stream_public_media@"public/media",
        "All public posts originating from this server."
        stream_local(flag only_media)@"public/local",
        "All public posts originating from other servers."
        stream_remote(flag only_media)@"public/remote",
        "All public posts using a certain hashtag."
        stream_hashtag(tag: impl AsRef<str>, like "#bots")@"hashtag",
        "All public posts using a certain hashtag, originating from this server."
        stream_local_hashtag(tag: impl AsRef<str>, like "#bots")@"hashtag/local",
        "Notifications for the current user."
        stream_notifications@"user/notification",
        "Updates to a specific list."
        stream_list(list: impl AsRef<str>, like "12345")@"list",
        "Updates to direct conversations."
        stream_direct@"direct",
    }

    /// A new instance.
    pub fn new(client: Client, data: Data) -> Self {
        Mastodon(Arc::new(MastodonClient { client, data }))
    }

    fn route(&self, url: impl AsRef<str>) -> String {
        format!("{}{}", self.data.base, url.as_ref())
    }

    /// POST /api/v1/filters
    pub async fn add_filter(&self, request: &mut AddFilterRequest) -> Result<Filter> {
        let response = self
            .client
            .post(self.route("/api/v1/filters"))
            .json(&request)
            .send()
            .await?;

        read_response(response).await
    }

    /// PUT /api/v1/filters/:id
    pub async fn update_filter(&self, id: &str, request: &mut AddFilterRequest) -> Result<Filter> {
        let url = self.route(format!("/api/v1/filters/{}", id));
        let response = self.client.put(&url).json(&request).send().await?;

        read_response(response).await
    }

    /// Update the user credentials
    pub async fn update_credentials(&self, changes: CredentialsBuilder) -> Result<Account> {
        let url = self.route("/api/v1/accounts/update_credentials");
        let response = self
            .client
            .patch(&url)
            .json(&changes.build()?)
            .send()
            .await?;

        read_response(response).await
    }

    /// Post a new status to the account.
    pub async fn new_status(&self, status: NewStatus) -> Result<Status> {
        let url = self.route("/api/v1/statuses");
        let response = self
            .authenticated(self.client.post(&url))
            .json(&status)
            .send()
            .await?;
        debug!(
            status = log_serde!(response Status), url = url,
            headers = log_serde!(response Headers);
            "received API response"
        );
        read_response(response).await
    }

    /// Get timeline filtered by a hashtag(eg. `#coffee`) either locally or
    /// federated.
    pub async fn get_tagged_timeline(&self, hashtag: String, local: bool) -> Result<Vec<Status>> {
        let base = "/api/v1/timelines/tag/";
        let url = if local {
            self.route(format!("{}{}?local=1", base, hashtag))
        } else {
            self.route(format!("{}{}", base, hashtag))
        };

        self.get(url).await
    }

    /// Get statuses of a single account by id. Optionally only with pictures
    /// and or excluding replies.
    ///
    /// // Example
    ///
    /// ```no_run
    /// use mastodon_async::prelude::*;
    /// tokio_test::block_on(async {
    ///     let data = Data::default();
    ///     let client = Mastodon::from(data);
    ///     let statuses = client.statuses(&AccountId::new("user-id"), Default::default()).await.unwrap();
    /// });
    /// ```
    ///
    /// ```no_run
    /// use mastodon_async::prelude::*;
    /// tokio_test::block_on(async {
    ///     let data = Data::default();
    ///     let client = Mastodon::from(data);
    ///     let mut request = StatusesRequest::new();
    ///     request.only_media();
    ///     let statuses = client.statuses(&AccountId::new("user-id"), request).await.unwrap();
    /// });
    /// ```
    pub async fn statuses<'a, 'b: 'a>(
        &'b self,
        id: &'b AccountId,
        request: StatusesRequest<'a>,
    ) -> Result<Page<Status>> {
        let call_id = Uuid::new_v4();
        let mut url = format!("{}/api/v1/accounts/{}/statuses", self.data.base, id);

        url += request.to_query_string()?.as_str();

        debug!(url = url, method = stringify!($method), call_id = as_debug!(call_id); "making API request");
        let response = self.client.get(&url).send().await?;

        Page::new(self.clone(), response, call_id).await
    }

    /// Returns the client account's relationship to a list of other accounts.
    /// Such as whether they follow them or vice versa.
    pub async fn relationships(&self, ids: &[&AccountId]) -> Result<Page<Relationship>> {
        let call_id = Uuid::new_v4();
        let mut url = self.route("/api/v1/accounts/relationships?");

        if ids.len() == 1 {
            url += "id=";
            url += ids[0].as_ref();
        } else {
            for id in ids {
                url += "id[]=";
                url += id.as_ref();
                url += "&";
            }
            url.pop();
        }

        debug!(
            url = url, method = stringify!($method),
            call_id = as_debug!(call_id), account_ids = as_serde!(ids);
            "making API request"
        );
        let response = self.client.get(&url).send().await?;

        Page::new(self.clone(), response, call_id).await
    }

    /// Add a push notifications subscription
    pub async fn add_push_subscription(&self, request: &AddPushRequest) -> Result<Subscription> {
        let call_id = Uuid::new_v4();
        let request = request.build()?;
        let url = &self.route("/api/v1/push/subscription");
        debug!(
            url = url, method = stringify!($method),
            call_id = as_debug!(call_id), post_body = as_serde!(request);
            "making API request"
        );
        let response = self.client.post(url).json(&request).send().await?;

        read_response(response).await
    }

    /// Update the `data` portion of the push subscription associated with this
    /// access token
    pub async fn update_push_data(&self, request: &UpdatePushRequest) -> Result<Subscription> {
        let call_id = Uuid::new_v4();
        let request = request.build();
        let url = &self.route("/api/v1/push/subscription");
        debug!(
            url = url, method = stringify!($method),
            call_id = as_debug!(call_id), post_body = as_serde!(request);
            "making API request"
        );
        let response = self.client.post(url).json(&request).send().await?;

        read_response(response).await
    }

    /// Get all accounts that follow the authenticated user
    pub async fn follows_me(&self) -> Result<Page<Account>> {
        let me = self.verify_credentials().await?;
        self.followers(&me.id).await
    }

    /// Get all accounts that the authenticated user follows
    pub async fn followed_by_me(&self) -> Result<Page<Account>> {
        let me = self.verify_credentials().await?;
        self.following(&me.id).await
    }

    /// Wait for the media to be done processing and return it with the URL.
    ///
    /// `Default::default()` may be passed as the polling time to select a
    /// polling time of 500ms.
    ///
    /// ## Example
    /// ```rust,no_run
    /// use mastodon_async::prelude::*;
    /// let mastodon = Mastodon::from(Data::default());
    /// tokio_test::block_on(async {
    ///     let attachment = mastodon.media("/path/to/some/file.jpg", None).await.expect("upload");
    ///     let attachment = mastodon.wait_for_processing(attachment, Default::default()).await.expect("processing");
    ///     println!("{}", attachment.url);
    /// });
    /// ```
    ///
    /// For a different polling time, use `.into()` on a `std::time::Duration`.
    /// ```rust,no_run
    /// use mastodon_async::prelude::*;
    /// use std::time::Duration;
    /// let mastodon = Mastodon::from(Data::default());
    /// tokio_test::block_on(async {
    ///     let attachment = mastodon.media("/path/to/some/file.jpg", None).await.expect("upload");
    ///     let attachment = mastodon.wait_for_processing(
    ///         attachment,
    ///         Duration::from_secs(1).into(),
    ///     ).await.expect("processing");
    ///     println!("{}", attachment.url);
    /// });
    /// ```
    pub async fn wait_for_processing(
        &self,
        mut attachment: Attachment,
        polling_time: PollingTime,
    ) -> Result<ProcessedAttachment> {
        let id = attachment.id;
        loop {
            if let Some(url) = attachment.url {
                return Ok(ProcessedAttachment {
                    id,
                    media_type: attachment.media_type,
                    url,
                    remote_url: attachment.remote_url,
                    preview_url: attachment.preview_url,
                    text_url: attachment.text_url,
                    meta: attachment.meta,
                    description: attachment.description,
                });
            } else {
                attachment = self.attachment(&id).await?;
                tokio::time::sleep(*polling_time).await;
            }
        }
    }

    /// Set the bearer authentication token
    fn authenticated(&self, request: RequestBuilder) -> RequestBuilder {
        request.bearer_auth(&self.data.token)
    }

    /// Return a part for a multipart form submission from a file, including
    /// the name of the file.
    fn get_form_part(path: impl AsRef<Path>) -> Result<Part> {
        use std::io::Read;

        let path = path.as_ref();

        match std::fs::File::open(path) {
            Ok(mut file) => {
                let mut data = if let Ok(metadata) = file.metadata() {
                    Vec::with_capacity(metadata.len().try_into()?)
                } else {
                    vec![]
                };
                file.read_to_end(&mut data)?;
                // TODO extract filename, error on dirs, etc.
                Ok(Part::bytes(data).file_name(Cow::Owned(path.to_string_lossy().to_string())))
            }
            Err(err) => {
                error!(path = as_debug!(path), error = as_debug!(err); "error reading file contents for multipart form");
                Err(err.into())
            }
        }
    }
}

impl MastodonUnauthenticated {
    methods![get and get_with_call_id,];

    /// Create a new client for unauthenticated requests to a given Mastodon
    /// instance.
    pub fn new(base: impl AsRef<str>) -> Result<MastodonUnauthenticated> {
        let base = base.as_ref();
        let base = if base.starts_with("https://") {
            base.to_string()
        } else {
            format!("https://{}", base.trim_start_matches("http://"))
        };
        trace!(base = base; "creating new mastodon client");
        Ok(MastodonUnauthenticated {
            client: Client::new(),
            base: Url::parse(&base)?,
        })
    }

    fn route(&self, url: &str) -> Result<Url> {
        Ok(self.base.join(url)?)
    }

    /// GET /api/v1/statuses/:id
    pub async fn get_status(&self, id: &StatusId) -> Result<Status> {
        let route = self.route("/api/v1/statuses")?;
        let route = route.join(id.as_ref())?;
        self.get(route.as_str()).await
    }

    /// GET /api/v1/statuses/:id/context
    pub async fn get_context(&self, id: &StatusId) -> Result<Context> {
        let route = self.route("/api/v1/statuses")?;
        let route = route.join(id.as_ref())?;
        let route = route.join("context")?;
        self.get(route.as_str()).await
    }

    /// GET /api/v1/statuses/:id/card
    pub async fn get_card(&self, id: &StatusId) -> Result<Card> {
        let route = self.route("/api/v1/statuses")?;
        let route = route.join(id.as_ref())?;
        let route = route.join("card")?;
        self.get(route.as_str()).await
    }

    /// Since this client needs no authentication, this returns the
    /// `RequestBuilder` unmodified.
    fn authenticated(&self, request: RequestBuilder) -> RequestBuilder {
        request
    }
}
impl Deref for Mastodon {
    type Target = Arc<MastodonClient>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<MastodonClient> for Mastodon {
    fn from(value: MastodonClient) -> Self {
        Mastodon(Arc::new(value))
    }
}
