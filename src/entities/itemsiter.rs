use futures::{stream::unfold, Stream};
use log::{as_debug, as_serde, debug, info, warn};

use crate::page::Page;
use serde::{Deserialize, Serialize};

/// Abstracts away the `next_page` logic into a single stream of items
///
/// ```no_run,async
/// use mastodon_async::prelude::*;
/// use futures::stream::StreamExt;
/// use futures_util::pin_mut;
///
/// tokio_test::block_on(async {
///     let data = Data::default();
///     let client = Mastodon::from(data);
///     let statuses = client.statuses("user-id", None).await.unwrap().items_iter();
///     statuses.for_each(|status| async move {
///         // Do something with the status
///     }).await;
/// })
/// ```
///
/// See documentation for `futures::Stream::StreamExt` for available methods.
#[derive(Debug, Clone)]
pub(crate) struct ItemsIter<T: Clone + for<'de> Deserialize<'de> + Serialize> {
    page: Page<T>,
    buffer: Vec<T>,
    cur_idx: usize,
    use_initial: bool,
}

impl<'a, T: Clone + for<'de> Deserialize<'de> + Serialize> ItemsIter<T> {
    pub(crate) fn new(page: Page<T>) -> ItemsIter<T> {
        ItemsIter {
            page,
            buffer: vec![],
            cur_idx: 0,
            use_initial: true,
        }
    }

    fn need_next_page(&self) -> bool {
        if self.buffer.is_empty() || self.cur_idx == self.buffer.len() {
            debug!(idx = self.cur_idx, buffer_len = self.buffer.len(); "next page needed");
            true
        } else {
            false
        }
    }

    async fn fill_next_page(&mut self) -> Option<()> {
        match self.page.next_page().await {
            Ok(Some(items)) => {
                info!(item_count = items.len(); "next page received");
                if items.is_empty() {
                    return None;
                }
                self.buffer = items;
                self.cur_idx = 0;
                Some(())
            },
            Err(err) => {
                warn!(err = as_debug!(err); "error encountered filling next page");
                None
            },
            _ => None,
        }
    }

    pub(crate) fn stream(self) -> impl Stream<Item = T> {
        unfold(self, |mut this| async move {
            if this.use_initial {
                let idx = this.cur_idx;
                if this.page.initial_items.is_empty() || idx == this.page.initial_items.len() {
                    debug!(index = idx, n_initial_items = this.page.initial_items.len(); "exhausted initial items and no more pages are present");
                    return None;
                }
                if idx == this.page.initial_items.len() - 1 {
                    this.cur_idx = 0;
                    this.use_initial = false;
                    debug!(index = idx, n_initial_items = this.page.initial_items.len(); "exhausted initial items");
                } else {
                    this.cur_idx += 1;
                }
                let item = this.page.initial_items[idx].clone();
                debug!(item = as_serde!(item), index = idx; "yielding item from initial items");
                // let item = Box::pin(item);
                // pin_mut!(item);
                Some((item, this))
            } else {
                if this.need_next_page() {
                    if this.fill_next_page().await.is_none() {
                        return None;
                    }
                }
                let idx = this.cur_idx;
                this.cur_idx += 1;
                let item = this.buffer[idx].clone();
                debug!(item = as_serde!(item), index = idx; "yielding item from initial stream");
                Some((item, this))
            }
        })
    }
}
