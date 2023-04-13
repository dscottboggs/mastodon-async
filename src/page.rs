use std::fmt::Debug;

use super::{Mastodon, Result};
use crate::{entities::itemsiter::ItemsIter, helpers::read_response::read_response, Error};
use futures::Stream;
use reqwest::{header::LINK, Response, Url};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, trace};
use uuid::Uuid;

macro_rules! pages {
    ($($direction:ident: $fun:ident),*) => {

        $(
            doc_comment!(concat!(
                    "Method to retrieve the ", stringify!($direction), " page of results",
                    "Returns Ok(None) if there is no data in the ", stringify!($direction), " page.\n",
                "Returns Ok(Some(Vec<T>)) if there are results.\n",
                "Returns Err(Error) if there is an error.\n",
                "If there are results, the next and previous page urls are stored.\n",
                "If there are no results, the next and previous page urls are not stored.\n",
                "This allows for the next page to be retrieved in the future even when\n",
                "there are no results.",
                ),
            pub async fn $fun(&mut self) -> Result<Option<Vec<T>>> {
                let Some(ref url) = self.$direction else {
                    return Ok(None);
                };

                debug!(
                    url = url.as_str(),
                    method = "get",
                    call_id = ?self.call_id,
                    direction = stringify!($direction),
                    "making API request"
                );
                let url: String = url.to_string();
                let response = self.mastodon.authenticated(self.mastodon.client.get(&url)).send().await?;
                match response.error_for_status() {
                    Ok(response) => {
                        let (prev, next) = get_links(&response, self.call_id)?;
                        let response: Vec<T> = read_response(response).await?;
                        if response.is_empty() && prev.is_none() && next.is_none() {
                            debug!(
                                url = url, method = "get", call_id = ?self.call_id,
                                direction = stringify!($direction),
                                "received an empty page with no links"
                            );
                            return Ok(None);
                        }
                        debug!(
                            url, method = "get", ?next,
                            ?prev, call_id = ?self.call_id,
                            direction = stringify!($direction),
                            response = ?response,
                            "received next pages from API"
                        );
                        self.next = next;
                        self.prev = prev;
                        Ok(Some(response))
                    }
                    Err(err) => {
                        error!(
                            ?err,
                            url,
                            method = stringify!($method),
                            call_id = ?self.call_id,
                            "error making API request"
                        );
                        Err(err.into())
                    }
                }

            });
         )*
    }
}

/// Owned version of the `Page` struct in this module. Allows this to be more
/// easily stored for later use
///
/// // Example
///
/// ```no_run
/// use mastodon_async::{
///     prelude::*,
///     page::Page,
///     entities::status::Status
/// };
/// use std::cell::RefCell;
///
/// tokio_test::block_on(async {
///     let data = Data::default();
///     struct HomeTimeline {
///         client: Mastodon,
///         page: RefCell<Option<Page<Status>>>,
///     }
///     let client = Mastodon::from(data);
///     let home = client.get_home_timeline().await.unwrap();
///     let tl = HomeTimeline {
///         client,
///         page: RefCell::new(Some(home)),
///     };
/// });
/// ```

/// Represents a single page of API results
#[derive(Debug, Clone)]
pub struct Page<T: for<'de> Deserialize<'de> + Serialize + Debug> {
    mastodon: Mastodon,
    /// next url
    pub next: Option<Url>,
    /// prev url
    pub prev: Option<Url>,
    /// Initial set of items
    pub initial_items: Vec<T>,
    pub(crate) call_id: Uuid,
}

impl<'a, T: for<'de> Deserialize<'de> + Serialize + Debug> Page<T> {
    pages! {
        next: next_page,
        prev: prev_page
    }

    /// Create a new Page.
    pub(crate) async fn new(mastodon: Mastodon, response: Response, call_id: Uuid) -> Result<Self> {
        let status = response.status();
        if status.is_success() {
            let (prev, next) = get_links(&response, call_id)?;
            let initial_items = read_response(response).await?;
            debug!(
                ?initial_items,
                ?prev,
                ?next,
                ?call_id,
                "received first page from API call"
            );
            Ok(Page {
                initial_items,
                next,
                prev,
                mastodon,
                call_id,
            })
        } else {
            let response = response.json().await?;
            Err(Error::Api { status, response })
        }
    }
}

impl<T: Clone + for<'de> Deserialize<'de> + Serialize + Debug> Page<T> {
    /// Returns an iterator that provides a stream of `T`s
    ///
    /// This abstracts away the process of iterating over each item in a page,
    /// then making an http call, then iterating over each item in the new
    /// page, etc. The iterator provides a stream of `T`s, calling
    /// `self.next_page()`
    /// when necessary to get
    /// more of them, until
    /// there are no more items.
    ///
    /// // Example
    ///
    /// ```no_run
    /// use mastodon_async::prelude::*;
    /// use futures_util::StreamExt;
    ///
    /// let data = Data::default();
    /// let mastodon = Mastodon::from(data);
    /// let req = StatusesRequest::new();
    ///
    /// tokio_test::block_on(async {
    ///     let resp = mastodon.statuses(&AccountId::new("some-id"), req).await.unwrap();
    ///     resp.items_iter().for_each(|status| async move {
    ///         // do something with status
    ///     }).await;
    /// });
    /// ```
    pub fn items_iter(self) -> impl Stream<Item = T> {
        ItemsIter::new(self).stream()
    }
}

fn get_links(response: &Response, call_id: Uuid) -> Result<(Option<Url>, Option<Url>)> {
    let mut prev = None;
    let mut next = None;

    if let Some(link_header) = response.headers().get(LINK) {
        let link_header = link_header.to_str()?;
        let raw_link_header = link_header.to_string();
        trace!(link_header = link_header, ?call_id, "parsing link header");
        let link_header = parse_link_header::parse(link_header)?;
        for (rel, link) in link_header.iter() {
            match rel.as_ref().map(|it| it.as_str()) {
                Some("next") => next = Some(link.uri.clone()),
                Some("prev") => prev = Some(link.uri.clone()),
                None => debug!(?link, "link header with no rel specified"),
                Some(other) => {
                    return Err(Error::UnrecognizedRel {
                        rel: other.to_string(),
                        link: raw_link_header,
                    })
                }
            }
        }
    }

    Ok((prev, next))
}
