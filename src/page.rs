use super::{Mastodon, Result};
use crate::{entities::itemsiter::ItemsIter, format_err};
use futures::Stream;
use hyper_old_types::header::{parsing, Link, RelationType};
use log::{as_debug, as_serde, debug, error, trace};
use reqwest::{header::LINK, Response, Url};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// use url::Url;

macro_rules! pages {
    ($($direction:ident: $fun:ident),*) => {

        $(
            doc_comment!(concat!(
                    "Method to retrieve the ", stringify!($direction), " page of results"),
            pub async fn $fun(&mut self) -> Result<Option<Vec<T>>> {
                let url = match self.$direction.take() {
                    Some(s) => s,
                    None => return Ok(None),
                };

                debug!(
                    url = as_debug!(url), method = "get",
                    call_id = as_debug!(self.call_id),
                    direction = stringify!($direction);
                    "making API request"
                );
                let url: String = url.into(); // <- for logging
                let response = self.mastodon.client.get(&url).send().await?;
                match response.error_for_status() {
                    Ok(response) => {
                        let (prev, next) = get_links(&response, self.call_id)?;
                        let response = response.json().await?;
                        debug!(
                            url = url, method = "get", next = as_debug!(next),
                            prev = as_debug!(prev), call_id = as_debug!(self.call_id),
                            response = as_serde!(response);
                            "received next pages from API"
                        );
                        self.next = next;
                        self.prev = prev;


                        Ok(Some(response))
                    }
                    Err(err) => {
                        error!(
                            err = as_debug!(err), url = url,
                            method = stringify!($method),
                            call_id = as_debug!(self.call_id);
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
/// use elefren::{
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
pub struct Page<T: for<'de> Deserialize<'de> + Serialize> {
    mastodon: Mastodon,
    next: Option<Url>,
    prev: Option<Url>,
    /// Initial set of items
    pub initial_items: Vec<T>,
    pub(crate) call_id: Uuid,
}

impl<'a, T: for<'de> Deserialize<'de> + Serialize> Page<T> {
    pages! {
        next: next_page,
        prev: prev_page
    }

    /// Create a new Page.
    pub(crate) async fn new(mastodon: Mastodon, response: Response, call_id: Uuid) -> Result<Self> {
        let (prev, next) = get_links(&response, call_id)?;
        let initial_items = response.json().await?;
        debug!(
            initial_items = as_serde!(initial_items), prev = as_debug!(prev),
            next = as_debug!(next), call_id = as_debug!(call_id);
            "received first page from API call"
        );
        Ok(Page {
            initial_items,
            next,
            prev,
            mastodon,
            call_id,
        })
    }
}

impl<T: Clone + for<'de> Deserialize<'de> + Serialize> Page<T> {
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
    /// use elefren::prelude::*;
    /// use futures_util::StreamExt;
    ///
    /// let data = Data::default();
    /// let mastodon = Mastodon::from(data);
    /// let req = StatusesRequest::new();
    ///
    /// tokio_test::block_on(async {
    ///     let resp = mastodon.statuses("some-id", req).await.unwrap();
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
        trace!(link_header = link_header, call_id = as_debug!(call_id); "parsing link header");
        let link_header = link_header.as_bytes();
        let link_header: Link = parsing::from_raw_str(&link_header)?;
        for value in link_header.values() {
            if let Some(relations) = value.rel() {
                if relations.contains(&RelationType::Next) {
                    // next = Some(Url::parse(value.link())?);
                    next = if let Ok(url) = Url::parse(value.link()) {
                        trace!(next = as_debug!(url), call_id = as_debug!(call_id); "parsed link header");
                        Some(url)
                    } else {
                        // HACK: url::ParseError::into isn't working for some reason.
                        return Err(format_err!("error parsing url {:?}", value.link()));
                    };
                }

                if relations.contains(&RelationType::Prev) {
                    prev = if let Ok(url) = Url::parse(value.link()) {
                        trace!(prev = as_debug!(url), call_id = as_debug!(call_id); "parsed link header");
                        Some(url)
                    } else {
                        // HACK: url::ParseError::into isn't working for some reason.
                        return Err(format_err!("error parsing url {:?}", value.link()));
                    };
                }
            }
        }
    }

    Ok((prev, next))
}
