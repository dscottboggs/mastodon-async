use std::{borrow::Cow, ops::Deref, sync::Arc};

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
use reqwest::Client;
use url::Url;

/// The Mastodon client is a smart pointer to this struct
#[derive(Clone, Debug)]
pub struct MastodonClient {
    pub(crate) client: Client,
    /// Raw data about your mastodon instance.
    pub data: Data,
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
    fn from(data: Data) -> Mastodon {
        MastodonClient {
            client: Client::new(),
            data,
        }
        .into()
    }
}
impl Mastodon {
    methods![get, post, delete,];

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
        (get  (q: &'a str, resolve: bool,)) search: "search" => SearchResult,
        (get  (local: bool,)) get_public_timeline: "timelines/public" => Vec<Status>,
        (post (uri: Cow<'static, str>,)) follows: "follows" => Account,
        (post multipart (file: Cow<'static, str>,)) media: "media" => Attachment,
        (post) clear_notifications: "notifications/clear" => Empty,
        (post (id: &str,)) dismiss_notification: "notifications/dismiss" => Empty,
        (get) get_push_subscription: "push/subscription" => Subscription,
        (delete) delete_push_subscription: "push/subscription" => Empty,
        (get) get_filters: "filters" => Vec<Filter>,
        (get) get_follow_suggestions: "suggestions" => Vec<Account>,
    }

    route_v2! {
        (get (q: &'a str, resolve: bool,)) search_v2: "search" => SearchResultV2,
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

    /// Create a new Mastodon Client
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
        Ok(self
            .client
            .post(self.route("/api/v1/filters"))
            .json(&request)
            .send()
            .await?
            .json()
            .await?)
    }

    /// PUT /api/v1/filters/:id
    pub async fn update_filter(&self, id: &str, request: &mut AddFilterRequest) -> Result<Filter> {
        let url = self.route(&format!("/api/v1/filters/{}", id));
        let response = self.client.put(&url).json(&request).send().await?;

        let status = response.status();

        if status.is_client_error() {
            return Err(Error::Client(status.clone()));
        } else if status.is_server_error() {
            return Err(Error::Server(status.clone()));
        }

        Ok(response.json().await?)
    }

    /// Update the user credentials
    pub async fn update_credentials(&self, builder: &mut UpdateCredsRequest) -> Result<Account> {
        let changes = builder.build()?;
        let url = self.route("/api/v1/accounts/update_credentials");
        let response = self.client.patch(&url).json(&changes).send().await?;

        let status = response.status();

        if status.is_client_error() {
            return Err(Error::Client(status.clone()));
        } else if status.is_server_error() {
            return Err(Error::Server(status.clone()));
        }

        Ok(response.json().await?)
    }

    /// Post a new status to the account.
    pub async fn new_status(&self, status: NewStatus) -> Result<Status> {
        Ok(self
            .client
            .post(&self.route("/api/v1/statuses"))
            .json(&status)
            .send()
            .await?
            .json()
            .await?)
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
    /// use elefren::prelude::*;
    /// tokio_test::block_on(async {
    ///     let data = Data::default();
    ///     let client = Mastodon::from(data);
    ///     let statuses = client.statuses("user-id", None).await.unwrap();
    /// });
    /// ```
    ///
    /// ```no_run
    /// use elefren::prelude::*;
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
        let mut url = format!("{}/api/v1/accounts/{}/statuses", self.data.base, id);

        if let Some(request) = request.into() {
            url = format!("{}{}", url, request.to_querystring()?);
        }

        let response = self.client.get(&url).send().await?;

        Page::new(self.clone(), response).await
    }

    /// Returns the client account's relationship to a list of other accounts.
    /// Such as whether they follow them or vice versa.
    pub async fn relationships(&self, ids: &[&str]) -> Result<Page<Relationship>> {
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

        let response = self.client.get(&url).send().await?;

        Page::new(self.clone(), response).await
    }

    /// Add a push notifications subscription
    pub async fn add_push_subscription(&self, request: &AddPushRequest) -> Result<Subscription> {
        let request = request.build()?;
        Ok(self
            .client
            .post(&self.route("/api/v1/push/subscription"))
            .json(&request)
            .send()
            .await?
            .json()
            .await?)
    }

    /// Update the `data` portion of the push subscription associated with this
    /// access token
    pub async fn update_push_data(&self, request: &UpdatePushRequest) -> Result<Subscription> {
        let request = request.build();
        Ok(self
            .client
            .put(&self.route("/api/v1/push/subscription"))
            .json(&request)
            .send()
            .await?
            .json()
            .await?)
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

    /// returns events that are relevant to the authorized user, i.e. home
    /// timeline & notifications
    ///
    /// // Example
    ///
    /// ```no_run
    /// use elefren::prelude::*;
    /// use elefren::entities::event::Event;
    /// use futures_util::{pin_mut, StreamExt, TryStreamExt};
    ///
    /// tokio_test::block_on(async {
    ///     let data = Data::default();
    ///     let client = Mastodon::from(data);
    ///     let stream = client.streaming_user().await.unwrap();
    ///     stream.try_for_each(|event| async move {
    ///         match event {
    ///             Event::Update(ref status) => { /* .. */ },
    ///             Event::Notification(ref notification) => { /* .. */ },
    ///             Event::Delete(ref id) => { /* .. */ },
    ///             Event::FiltersChanged => { /* .. */ },
    ///         }
    ///         Ok(())
    ///     }).await.unwrap();
    /// });
    /// ```
    pub async fn streaming_user(&self) -> Result<impl TryStream<Ok = Event, Error = Error>> {
        let mut url: Url = self.route("/api/v1/streaming").parse()?;
        url.query_pairs_mut()
            .append_pair("access_token", &self.data.token)
            .append_pair("stream", "user");
        let mut url: Url = reqwest::get(url.as_str()).await?.url().as_str().parse()?;
        let new_scheme = match url.scheme() {
            "http" => "ws",
            "https" => "wss",
            x => return Err(Error::Other(format!("Bad URL scheme: {}", x))),
        };
        url.set_scheme(new_scheme)
            .map_err(|_| Error::Other("Bad URL scheme!".to_string()))?;

        event_stream(url)
    }
}

impl MastodonUnauthenticated {
    methods![get,];

    /// Create a new client for unauthenticated requests to a given Mastodon
    /// instance.
    pub fn new(base: impl AsRef<str>) -> Result<MastodonUnauthenticated> {
        let base = base.as_ref();
        let base = if base.starts_with("https://") {
            base.to_string()
        } else {
            format!("https://{}", base.trim_start_matches("http://"))
        };
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
