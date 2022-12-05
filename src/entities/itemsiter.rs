use futures::{stream::unfold, Stream};

use crate::page::Page;
use serde::Deserialize;

/// Abstracts away the `next_page` logic into a single stream of items
///
/// ```no_run,async
/// use elefren::prelude::*;
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
pub(crate) struct ItemsIter<T: Clone + for<'de> Deserialize<'de>> {
    page: Page<T>,
    buffer: Vec<T>,
    cur_idx: usize,
    use_initial: bool,
}

impl<'a, T: Clone + for<'de> Deserialize<'de>> ItemsIter<T> {
    pub(crate) fn new(page: Page<T>) -> ItemsIter<T> {
        ItemsIter {
            page,
            buffer: vec![],
            cur_idx: 0,
            use_initial: true,
        }
    }

    fn need_next_page(&self) -> bool {
        self.buffer.is_empty() || self.cur_idx == self.buffer.len()
    }

    async fn fill_next_page(&mut self) -> Option<()> {
        let items = if let Ok(items) = self.page.next_page().await {
            items
        } else {
            return None;
        };
        if let Some(items) = items {
            if items.is_empty() {
                return None;
            }
            self.buffer = items;
            self.cur_idx = 0;
            Some(())
        } else {
            None
        }
    }

    pub(crate) fn stream(self) -> impl Stream<Item = T> {
        unfold(self, |mut this| async move {
            if this.use_initial {
                if this.page.initial_items.is_empty()
                    || this.cur_idx == this.page.initial_items.len()
                {
                    return None;
                }
                let idx = this.cur_idx;
                if this.cur_idx == this.page.initial_items.len() - 1 {
                    this.cur_idx = 0;
                    this.use_initial = false;
                } else {
                    this.cur_idx += 1;
                }
                let item = this.page.initial_items[idx].clone();
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
                // let item = Box::pin(item);
                // pin_mut!(item);
                Some((item, this))
            }
        })
    }
}
