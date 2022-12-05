use super::{Mastodon, Result};
use crate::{entities::itemsiter::ItemsIter, format_err};
use futures::Stream;
use hyper_old_types::header::{parsing, Link, RelationType};
use reqwest::{header::LINK, Response, Url};
use serde::Deserialize;
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

                let response = self.mastodon.client.get(url).send().await?;

                let (prev, next) = get_links(&response)?;
                self.next = next;
                self.prev = prev;

                Ok(Some(response.json().await?))
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
pub struct Page<T: for<'de> Deserialize<'de>> {
    mastodon: Mastodon,
    next: Option<Url>,
    prev: Option<Url>,
    /// Initial set of items
    pub initial_items: Vec<T>,
}

impl<'a, T: for<'de> Deserialize<'de>> Page<T> {
    pages! {
        next: next_page,
        prev: prev_page
    }

    /// Create a new Page.
    pub(crate) async fn new(mastodon: Mastodon, response: Response) -> Result<Self> {
        let (prev, next) = get_links(&response)?;
        Ok(Page {
            initial_items: response.json().await?,
            next,
            prev,
            mastodon,
        })
    }
}

impl<T: Clone + for<'de> Deserialize<'de>> Page<T> {
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

fn get_links(response: &Response) -> Result<(Option<Url>, Option<Url>)> {
    let mut prev = None;
    let mut next = None;

    if let Some(link_header) = response.headers().get(LINK) {
        let link_header = link_header.to_str()?;
        let link_header = link_header.as_bytes();
        let link_header: Link = parsing::from_raw_str(&link_header)?;
        for value in link_header.values() {
            if let Some(relations) = value.rel() {
                if relations.contains(&RelationType::Next) {
                    // next = Some(Url::parse(value.link())?);
                    next = if let Ok(url) = Url::parse(value.link()) {
                        Some(url)
                    } else {
                        // HACK: url::ParseError::into isn't working for some reason.
                        return Err(format_err!("error parsing url {:?}", value.link()));
                    };
                }

                if relations.contains(&RelationType::Prev) {
                    prev = if let Ok(url) = Url::parse(value.link()) {
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
