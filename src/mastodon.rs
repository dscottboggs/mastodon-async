use std::{borrow::Cow, ops::Deref, path::Path, sync::Arc};

use crate::{
    as_value,
    entities::{
        account::Account, application::AuthenticatedApplication, attachment::ProcessedAttachment,
        prelude::*, report::Report, status::Status, Empty,
    },
    errors::{Error, Result},
    helpers::read_response::read_response,
    polling_time::PollingTime,
    AddFilterRequest, AddPushRequest, Data, NewStatus, Page, UpdatePushRequest,
};
use futures::TryStream;
use reqwest::{multipart::Part, Client, RequestBuilder};
use tracing::{debug, error, trace};
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
    pub base: String,
}

impl From<Data> for Mastodon {
    /// Creates a mastodon instance from the data struct.
    #[tracing::instrument(skip(data))]
    fn from(data: Data) -> Mastodon {
        Mastodon::new(Client::new(), data)
    }
}
impl Mastodon {
    methods![get and get_with_call_id, post and post_with_call_id, delete and delete_with_call_id,];

    paged_routes! {
        (get) favourites: "favourites" => Status,
        (get) bookmarks: "bookmarks" => Status,
        (get) blocks: "blocks" => Account,
        (get) domain_blocks: "domain_blocks" => String,
        (get) instance_domain_blocks: "instance/domain_blocks" => DomainBlock,
        (get) follow_requests: "follow_requests" => Account,
        (get) get_home_timeline: "timelines/home" => Status,
        (get  (local: bool,)) get_public_timeline: "timelines/public" => Status,
        (get) get_emojis: "custom_emojis" => CustomEmoji,
        (get) mutes: "mutes" => Account,
        (get) notifications: "notifications" => Notification,
        (get) instance_peers: "instance/peers" => String,
        (get) instance_activity: "instance/activity" => instance::Activity,
        (get) instance_rules: "instance/rules" => instance::Rule,
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
        (post (uri: Cow<'static, str>,)) follows: "follows" => Account,
        (post) clear_notifications: "notifications/clear" => Empty,
        (post (id: &str,)) dismiss_notification: "notifications/dismiss" => Empty,
        (get) get_push_subscription: "push/subscription" => Subscription,
        (delete) delete_push_subscription: "push/subscription" => Empty,
        (get) get_filters: "filters" => Vec<Filter>,
        (get) get_follow_suggestions: "suggestions" => Vec<Account>,
        (post (app: forms::Application,)) create_app: "apps" => Application,
        (get) verify_app: "apps/verify_credentials" => Application,
    }

    route_v2! {
        (get (q: &'a str, resolve: bool,)) search: "search" => SearchResult,
        (post multipart with description (file: impl AsRef<Path>,)) media: "media" => Attachment,
        (post multipart with description (file: impl AsRef<Path>, thumbnail: impl AsRef<Path>,)) media_with_thumbnail: "media" => Attachment,
    }

    route_id! {
        (get) get_account[AccountId]: "accounts/{}" => Account,
        (post) unfollow[AccountId]: "accounts/{}/unfollow" => Relationship,
        (post) remove_from_followers[AccountId]: "accounts/{}/remove_from_followers" => Relationship,
        (post) block[AccountId]: "accounts/{}/block" => Relationship,
        (post) unblock[AccountId]: "accounts/{}/unblock" => Relationship,
        (get) mute[AccountId]: "accounts/{}/mute" => Relationship,
        (get) unmute[AccountId]: "accounts/{}/unmute" => Relationship,
        (post) feature_account[AccountId]: "accounts/{}/pin" => Relationship,
        (post) stop_featuring_account[AccountId]: "accounts/{}/unpin" => Relationship,
        (get) featured_tags[AccountId]: "accounts/{}/featured_tags" => Vec<tag::Featured>,
        (get) in_lists[AccountId]: "accounts/{}/lists" => Vec<List>,
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

    query_form_route! {
        "Obtain an access token, to be used during API calls that are not public."
        (post token_form: forms::oauth::TokenRequest) get_auth_token: "oauth/token" => Token,
        "Returns the client account's relationship to a list of other accounts. \
         Such as whether they follow them or vice versa."
        (get ids: forms::account::IdList) relationships: "accounts/relationships" => Vec<Relationship>,
        "Obtain a list of all accounts that follow a given account, filtered for accounts you follow."
        (get ids: forms::account::IdList) familiar_followers: "accounts/familiar_followers" => Vec<Relationship>,
    }

    /// A new instance.
    pub fn new(client: Client, data: Data) -> Self {
        Mastodon(Arc::new(MastodonClient { client, data }))
    }

    fn route(&self, url: impl AsRef<str>) -> String {
        format!("{}{}", self.data.base, url.as_ref())
    }

    /// POST /api/v1/filters
    pub async fn add_filter(&self, request: &AddFilterRequest) -> Result<Filter> {
        let response = self
            .client
            .post(self.route("/api/v1/filters"))
            .json(&request)
            .send()
            .await?;

        read_response(response).await
    }

    /// PUT /api/v1/filters/:id
    pub async fn update_filter(&self, id: &str, request: &AddFilterRequest) -> Result<Filter> {
        let url = self.route(format!("/api/v1/filters/{id}"));
        let response = self.client.put(&url).json(&request).send().await?;

        read_response(response).await
    }

    post_route! {
        "Update the user credentials"
        [patch] update_credentials(account::Credentials)@"accounts/update_credentials" -> Account,
        "Post a new status to the account."
        [post] new_status(NewStatus)@"statuses" -> Status,
        "Creates a user and account records."
        [post] create_account(forms::account::Creation)@"accounts" -> Token,
    }

    ///Revoke an access token to make it no longer valid for use.
    pub async fn revoke_auth(
        &self,
        post_body: forms::oauth::token::Revocation,
    ) -> Result<auth::RevocationResponse> {
        let url = self.route("/oauth/revoke");
        let response = self.client.post(&url).json(&post_body).send().await?;
        read_response(response).await
    }
    /// Edit existing status
    pub async fn update_status(&self, id: &StatusId, status: NewStatus) -> Result<Status> {
        let url = self.route(format!("/api/v1/statuses/{id}"));
        let response = self
            .authenticated(self.client.put(&url))
            .json(&status)
            .send()
            .await?;
        debug!(
            response = as_value!(response, Response), updated_status_id = ?id, ?status,
            "received API response"
        );
        read_response(response).await
    }

    /// Add a private note about an account
    pub async fn add_note_to_account(
        &self,
        account: AccountId,
        comment: String,
    ) -> Result<Relationship> {
        #[derive(Serialize)]
        struct Note {
            comment: String,
        }
        let url = self.route(format!("/api/v1/accounts/{account}/note"));
        let note = Note { comment };
        let response = self
            .authenticated(self.client.post(&url))
            .json(&note)
            .send()
            .await?;
        debug!(response = as_value!(response, Response), ?account, comment=?note.comment, "received API response");
        read_response(response).await
    }

    /// Follow an account, or update your follow preferences
    pub async fn follow(
        &self,
        id: &AccountId,
        options: forms::account::FollowOptions,
    ) -> Result<Relationship> {
        let url = self.route(format!(
            "/api/v1/accounts/{id}/follow?{}",
            serde_qs::to_string(&options)?
        ));
        let response = self.authenticated(self.client.post(&url)).send().await?;
        debug!(
            response = as_value!(response, Response), followed_account = ?id, ?options,
            "received API response"
        );
        read_response(response).await
    }

    /// Get timeline filtered by a hashtag(eg. `#coffee`) either locally or
    /// federated.
    pub async fn get_tagged_timeline(&self, hashtag: String, local: bool) -> Result<Vec<Status>> {
        let base = "/api/v1/timelines/tag/";
        let url = if local {
            self.route(format!("{base}{hashtag}?local=1"))
        } else {
            self.route(format!("{base}{hashtag}"))
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
    ///     let mut options = forms::status_request::Options::builder();
    ///     options.only_media(true);
    ///     let statuses = client.statuses(&AccountId::new("user-id"), options.build()).await.unwrap();
    /// });
    /// ```
    pub async fn statuses(
        &self,
        id: &AccountId,
        options: forms::status_request::Options,
    ) -> Result<Page<Status>> {
        let call_id = Uuid::new_v4();
        let mut url = format!("{}/api/v1/accounts/{id}/statuses", self.data.base);

        url += options.to_query_string()?.as_str();

        debug!(
            url,
            method = stringify!($method),
            ?call_id,
            "making API request"
        );
        let response = self.client.get(&url).send().await?;

        Page::new(self.clone(), response, call_id).await
    }

    /// Add a push notifications subscription
    pub async fn add_push_subscription(&self, request: &AddPushRequest) -> Result<Subscription> {
        let call_id = Uuid::new_v4();
        let request = request.build();
        let url = &self.route("/api/v1/push/subscription");
        debug!(
            url, method = stringify!($method),
            ?call_id, post_body = ?request,
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
            url = url, method = "post", ?call_id, post_body = ?request,
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
    pub(crate) fn authenticated(&self, request: RequestBuilder) -> RequestBuilder {
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
                error!(?path, error = ?err, "error reading file contents for multipart form");
                Err(err.into())
            }
        }
    }

    /// Access public (unauthenticated) endpoints.
    ///
    /// This clones this client's client and base URL, so if it's possible,
    /// keep the pre-authentication [`MastodonUnauthenticated`] around instead.
    pub fn public_api(&self) -> MastodonUnauthenticated {
        MastodonUnauthenticated {
            client: self.client.clone(),
            base: self.data.base.clone(),
        }
    }
}

impl MastodonUnauthenticated {
    methods![get and get_with_call_id, post and post_with_call_id, ];

    /// Create a new client for unauthenticated requests to a given Mastodon
    /// instance.
    pub fn new(base: impl AsRef<str>) -> Result<Self> {
        let base = base.as_ref();
        let base = if base.starts_with("https://") {
            base.to_string()
        } else {
            format!("https://{}", base.trim_start_matches("http://"))
        };
        trace!(base = base, "creating new mastodon client");
        Ok(Self {
            client: Client::new(),
            base,
        })
    }

    fn route(&self, url: impl std::fmt::Display) -> String {
        format!("{}{url}", self.base.as_str())
    }

    route! {
        (post (app: forms::Application,)) create_app: "apps" => AuthenticatedApplication,
    }

    /// GET /api/v1/statuses/:id
    pub async fn get_status(&self, id: &StatusId) -> Result<Status> {
        let route = self.route(format!("/api/v1/statuses/{id}"));
        self.get(route.as_str()).await
    }

    /// GET /api/v1/statuses/:id/context
    pub async fn get_context(&self, id: &StatusId) -> Result<Context> {
        let route = self.route(format!("/api/v1/statuses/{id}/context"));
        self.get(route.as_str()).await
    }

    /// GET /api/v1/statuses/:id/card
    pub async fn get_card(&self, id: &StatusId) -> Result<Card> {
        let route = self.route(format!("/api/v1/statuses/{id}/card"));
        self.get(route.as_str()).await
    }

    /// Since this client needs no authentication, this returns the
    /// `RequestBuilder` unmodified.
    fn authenticated(&self, request: RequestBuilder) -> RequestBuilder {
        request
    }

    /// Return an authenticated [`Mastodon`] client.
    pub fn authorized(&self, app: AuthenticatedApplication, oauth_token: OAuthToken) -> Mastodon {
        let data = Data::builder(
            self.base.clone(),
            app.client_id,
            app.client_secret,
            oauth_token,
        )
        .build();
        Mastodon::new(reqwest::Client::new(), data)
    }

    /// Displays an authorization form to the user. If approved, it will create
    /// and return an authorization code, then redirect to the desired
    /// redirect_uri, or show the authorization code if urn:ietf:wg:oauth:2.0:oob
    /// was requested. The authorization code can be used while requesting a
    /// token to obtain access to user-level methods.
    ///
    /// ### Response
    /// The authorization code will be returned as a query parameter named code.
    ///
    /// ```text
    /// redirect_uri?code=qDFUEaYrRK5c-HNmTCJbAzazwLRInJ7VHFat0wcMgCU
    /// ```
    ///
    /// See also [the API reference](https://docs.joinmastodon.org/methods/oauth/#authorize)
    pub async fn request_oauth_authorization(
        &self,
        request: forms::oauth::AuthorizationRequest,
    ) -> Result<String> {
        let call_id = Uuid::new_v4();
        let query = serde_qs::to_string(&request)?;
        let url = self.route(format!("/oauth/authorize?{query}"));
        debug!(?url, method = "get", ?call_id, ?query, "making API request");
        let response = self.client.get(url).send().await?;
        trace!(
            response = as_value!(response, Response),
            "API response received"
        );
        Ok(response.text().await?)
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
