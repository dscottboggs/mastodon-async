use std::fmt::Debug;

use super::{Mastodon, Result};
use crate::{entities::itemsiter::ItemsIter, helpers::read_response::read_response, Error};
use futures::Stream;
use reqwest::{header::LINK, Response, Url};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, trace};
use uuid::Uuid;

macro_rules! pages {
    ($($direction:ident: $method:ident),*) => {

        $(
            doc_comment!(concat!(
                    "Method to retrieve the ", stringify!($direction), " page of results"),
            #[tracing::instrument(skip(self), fields(call_id = %Uuid::new_v4()))]
            pub async fn $method(&mut self) -> Result<Option<Vec<T>>> {
                let url = match self.$direction.take() {
                    Some(s) => s,
                    None => return Ok(None),
                };

                debug!(method = "get", url = url.as_str(), direction = stringify!($direction), "making API request");
                let url: String = url.into(); // <- for logging
                let response = self.mastodon.client.get(&url).send().await?;
                match response.error_for_status() {
                    Ok(response) => {
                        let (prev, next) = get_links(&response)?;
                        let response = read_response(response).await?;
                        debug!(method = "get", url, ?next, ?prev, response = ?response, "received next pages from API");
                        self.next = next;
                        self.prev = prev;

                        Ok(Some(response))
                    }
                    Err(err) => {
                        error!( ?err, method = "get", url, "error making API request" );
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
    next: Option<Url>,
    prev: Option<Url>,
    /// Initial set of items
    pub initial_items: Vec<T>,
}

impl<'a, T: for<'de> Deserialize<'de> + Serialize + Debug> Page<T> {
    pages! {
        next: next_page,
        prev: prev_page
    }

    /// Create a new Page.
    pub(crate) async fn new(mastodon: Mastodon, response: Response) -> Result<Self> {
        let status = response.status();
        if status.is_success() {
            let (prev, next) = get_links(&response)?;
            let initial_items = read_response(response).await?;
            debug!(
                ?prev,
                ?next,
                ?initial_items,
                "received first page from API call"
            );
            Ok(Page {
                initial_items,
                next,
                prev,
                mastodon,
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

fn get_links(response: &Response) -> Result<(Option<Url>, Option<Url>)> {
    let mut prev = None;
    let mut next = None;

    if let Some(link_header) = response.headers().get(LINK) {
        let link_header = link_header.to_str()?;
        let raw_link_header = link_header.to_string();
        trace!(%link_header, "parsing link header");
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
