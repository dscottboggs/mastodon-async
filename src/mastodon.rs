use std::{borrow::Cow, ops::Deref, path::Path, sync::Arc};

use crate::{
    entities::{
        account::Account,
        prelude::*,
        report::Report,
        status::{Emoji, Status},
        Empty,
    },
    errors::{Error, Result},
    event_stream::event_stream,
    helpers::read_response::read_response,
    log_serde,
    AddFilterRequest,
    AddPushRequest,
    Data,
    NewStatus,
    Page,
    StatusesRequest,
    UpdateCredsRequest,
    UpdatePushRequest,
};
use futures::TryStream;
use log::{as_debug, as_serde, debug, error, trace};
#[cfg(feature = "magic")]
use magic::CookieFlags;
use reqwest::{multipart::Part, Client, RequestBuilder};
use url::Url;
use uuid::Uuid;

/// The Mastodon client is a smart pointer to this struct
#[derive(Debug)]
pub struct MastodonClient {
    pub(crate) client: Client,
    /// Raw data about your mastodon instance.
    pub data: Data,
    /// A handle to access libmagic for mime-types.
    #[cfg(feature = "magic")]
    magic: magic::Cookie,
}

/// Your mastodon application client, handles all requests to and from Mastodon.
#[derive(Debug, Clone)]
pub struct Mastodon(Arc<MastodonClient>);

/// A client for making unauthenticated requests to the public API.
#[derive(Clone, Debug)]
pub struct MastodonUnauthenticated {
    client: Client,
    /// Which Mastodon instance to contact
    pub base: Url,
}

impl From<Data> for Mastodon {
    /// Creates a mastodon instance from the data struct.
    #[cfg(feature = "magic")]
    fn from(data: Data) -> Mastodon {
        MastodonClient {
            client: Client::new(),
            data,
            magic: Self::default_magic().expect("failed to open magic cookie or load database"),
        }
        .into()
    }

    /// Creates a mastodon instance from the data struct.
    #[cfg(not(feature = "magic"))]
    fn from(data: Data) -> Mastodon {
        MastodonClient {
            client: Client::new(),
            data,
        }
        .into()
    }
}
impl Mastodon {
    methods![get and get_with_call_id, post and post_with_call_id, delete and delete_with_call_id,];

    paged_routes! {
        (get) favourites: "favourites" => Status,
        (get) blocks: "blocks" => Account,
        (get) domain_blocks: "domain_blocks" => String,
        (get) follow_requests: "follow_requests" => Account,
        (get) get_home_timeline: "timelines/home" => Status,
        (get) get_emojis: "custom_emojis" => Emoji,
        (get) mutes: "mutes" => Account,
        (get) notifications: "notifications" => Notification,
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
        (get) get_account: "accounts/{}" => Account,
        (post) follow: "accounts/{}/follow" => Relationship,
        (post) unfollow: "accounts/{}/unfollow" => Relationship,
        (post) block: "accounts/{}/block" => Relationship,
        (post) unblock: "accounts/{}/unblock" => Relationship,
        (get) mute: "accounts/{}/mute" => Relationship,
        (get) unmute: "accounts/{}/unmute" => Relationship,
        (get) get_notification: "notifications/{}" => Notification,
        (get) get_status: "statuses/{}" => Status,
        (get) get_context: "statuses/{}/context" => Context,
        (get) get_card: "statuses/{}/card" => Card,
        (post) reblog: "statuses/{}/reblog" => Status,
        (post) unreblog: "statuses/{}/unreblog" => Status,
        (post) favourite: "statuses/{}/favourite" => Status,
        (post) unfavourite: "statuses/{}/unfavourite" => Status,
        (delete) delete_status: "statuses/{}" => Empty,
        (get) get_filter: "filters/{}" => Filter,
        (delete) delete_filter: "filters/{}" => Empty,
        (delete) delete_from_suggestions: "suggestions/{}" => Empty,
        (post) endorse_user: "accounts/{}/pin" => Relationship,
        (post) unendorse_user: "accounts/{}/unpin" => Relationship,
    }

    streaming! {
        "returns events that are relevant to the authorized user, i.e. home timeline & notifications"
        stream_user@"user",
        "All public posts known to the server. Analogous to the federated timeline."
        stream_public@"public",
        "All public posts known to the server, filtered for media attachments. Analogous to the federated timeline with 'only media' enabled."
        stream_public_media@"public:media",
        "All public posts originating from this server."
        stream_local@"public:local",
        "All public posts originating from this server, filtered for media attachments. Analogous to the local timeline with 'only media' enabled."
        stream_local_media@"public:local:media",
        "All public posts originating from other servers."
        stream_remote@"public:remote",
        "All public posts originating from other servers, filtered for media attachments."
        stream_remote_media@"public:remote:media",
        "All public posts using a certain hashtag."
        stream_hashtag(tag: impl AsRef<str>, like "#bots")@"hashtag",
        "All public posts using a certain hashtag, originating from this server."
        stream_local_hashtag(tag: impl AsRef<str>, like "#bots")@"hashtag:local",
        "Notifications for the current user."
        stream_notifications@"user:notification",
        "Updates to a specific list."
        stream_list(list: impl AsRef<str>, like "12345")@"list",
        "Updates to direct conversations."
        stream_direct@"direct",
    }

    /// Return a magic cookie, loaded with the default mime
    #[cfg(feature = "magic")]
    fn default_magic() -> Result<magic::Cookie> {
        let magic = magic::Cookie::open(Default::default())?;
        magic.load::<&str>(&[])?;
        magic.set_flags(CookieFlags::MIME)?;
        Ok(magic)
    }

    /// Create a new Mastodon Client
    #[cfg(feature = "magic")]
    pub fn new(client: Client, data: Data) -> Self {
        Self::new_with_magic(
            client,
            data,
            Self::default_magic().expect("failed to open magic cookie or load database"),
        )
    }

    /// Create a new Mastodon Client, passing in a pre-constructed magic
    /// cookie.
    ///
    /// This is mainly here so you have a wait to construct the client which
    /// won't panic.
    #[cfg(feature = "magic")]
    pub fn new_with_magic(client: Client, data: Data, magic: magic::Cookie) -> Self {
        Mastodon(Arc::new(MastodonClient {
            client,
            data,
            magic,
        }))
    }

    /// Create a new Mastodon Client
    #[cfg(not(feature = "magic"))]
    pub fn new(client: Client, data: Data) -> Self {
        Mastodon(Arc::new(MastodonClient {
            client,
            data,
        }))
    }

    fn route(&self, url: &str) -> String {
        format!("{}{}", self.data.base, url)
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
        let url = self.route(&format!("/api/v1/filters/{}", id));
        let response = self.client.put(&url).json(&request).send().await?;

        read_response(response).await
    }

    /// Update the user credentials
    pub async fn update_credentials(&self, builder: &mut UpdateCredsRequest) -> Result<Account> {
        let changes = builder.build()?;
        let url = self.route("/api/v1/accounts/update_credentials");
        let response = self.client.patch(&url).json(&changes).send().await?;

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
            self.route(&format!("{}{}?local=1", base, hashtag))
        } else {
            self.route(&format!("{}{}", base, hashtag))
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
    ///     let statuses = client.statuses("user-id", None).await.unwrap();
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
    ///     let statuses = client.statuses("user-id", request).await.unwrap();
    /// });
    /// ```
    pub async fn statuses<'a, 'b: 'a, S>(&'b self, id: &'b str, request: S) -> Result<Page<Status>>
    where
        S: Into<Option<StatusesRequest<'a>>>,
    {
        let call_id = Uuid::new_v4();
        let mut url = format!("{}/api/v1/accounts/{}/statuses", self.data.base, id);

        if let Some(request) = request.into() {
            url = format!("{}{}", url, request.to_querystring()?);
        }

        debug!(url = url, method = stringify!($method), call_id = as_debug!(call_id); "making API request");
        let response = self.client.get(&url).send().await?;

        Page::new(self.clone(), response, call_id).await
    }

    /// Returns the client account's relationship to a list of other accounts.
    /// Such as whether they follow them or vice versa.
    pub async fn relationships(&self, ids: &[&str]) -> Result<Page<Relationship>> {
        let call_id = Uuid::new_v4();
        let mut url = self.route("/api/v1/accounts/relationships?");

        if ids.len() == 1 {
            url += "id=";
            url += &ids[0];
        } else {
            for id in ids {
                url += "id[]=";
                url += &id;
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

    /// Set the bearer authentication token
    fn authenticated(&self, request: RequestBuilder) -> RequestBuilder {
        request.bearer_auth(&self.data.token)
    }

    /// Return a part for a multipart form submission from a file, including
    /// the name of the file, and, if the "magic" feature is enabled, the mime-
    /// type.
    fn get_form_part(&self, path: impl AsRef<Path>) -> Result<Part> {
        use std::io::Read;

        let path = path.as_ref();

        // if it doesn't work, it's no big deal. The server will look at
        // the filepath if this isn't here and things should still work.
        #[cfg(feature = "magic")]
        let mime = self.magic.file(path).ok();
        #[cfg(not(feature = "magic"))]
        let mime: Option<String> = None;

        match std::fs::File::open(path) {
            Ok(mut file) => {
                let mut data = if let Ok(metadata) = file.metadata() {
                    Vec::with_capacity(metadata.len().try_into()?)
                } else {
                    vec![]
                };
                file.read_to_end(&mut data)?;
                let part =
                    Part::bytes(data).file_name(Cow::Owned(path.to_string_lossy().to_string()));
                Ok(if let Some(mime) = mime {
                    part.mime_str(&mime)?
                } else {
                    part
                })
            },
            Err(err) => {
                error!(path = as_debug!(path), error = as_debug!(err); "error reading file contents for multipart form");
                return Err(err.into());
            },
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
    pub async fn get_status(&self, id: &str) -> Result<Status> {
        let route = self.route("/api/v1/statuses")?;
        let route = route.join(id)?;
        self.get(route.as_str()).await
    }

    /// GET /api/v1/statuses/:id/context
    pub async fn get_context(&self, id: &str) -> Result<Context> {
        let route = self.route("/api/v1/statuses")?;
        let route = route.join(id)?;
        let route = route.join("context")?;
        self.get(route.as_str()).await
    }

    /// GET /api/v1/statuses/:id/card
    pub async fn get_card(&self, id: &str) -> Result<Card> {
        let route = self.route("/api/v1/statuses")?;
        let route = route.join(id)?;
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
