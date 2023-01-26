//! # mastodon-async: API Wrapper around the Mastodon API.
//!
//! Most of the api is documented on [Mastodon's website](https://docs.joinmastodon.org/client/intro/)
//!
//! ```no_run
//! use mastodon_async::{helpers::cli, prelude::*};
//! use futures_util::StreamExt;
//!
//! tokio_test::block_on(async {
//!     let registration = Registration::new("https://botsin.space")
//!         .client_name("mastodon-async_test")
//!         .build()
//!         .await
//!         .unwrap();
//!     let mastodon = cli::authenticate(registration).await.unwrap();
//!
//!     println!(
//!         "{:?}",
//!         mastodon
//!             .get_home_timeline()
//!             .await
//!             .unwrap()
//!             .items_iter()
//!             .take(100)
//!             .collect::<Vec<_>>()
//!             .await
//!     );
//! });
//! ```
//!
//! mastodon-async also supports Mastodon's Streaming API:
//!
//! ## Example
//!
//! ```no_run
//! use mastodon_async::{prelude::*, entities::event::Event};
//! use futures_util::TryStreamExt;
//!
//! let data = Data::default();
//! let client = Mastodon::from(data);
//! tokio_test::block_on(async {
//!     let stream = client.stream_user().await.unwrap();
//!     stream.try_for_each(|event| async move {
//!         match event {
//!             Event::Update(ref status) => { /* .. */ },
//!             Event::Notification(ref notification) => { /* .. */ },
//!             Event::Delete(ref id) => { /* .. */ },
//!             Event::FiltersChanged => { /* .. */ },
//!         }
//!         Ok(())
//!     }).await.unwrap();
//! });
//! ```

#![deny(
    missing_docs,
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

#[macro_use]
extern crate doc_comment;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde;

#[cfg(feature = "env")]
extern crate envy;

#[cfg(feature = "toml")]
extern crate toml as tomlcrate;

#[cfg(test)]
extern crate tempfile;

#[cfg(test)]
#[cfg_attr(all(test, any(feature = "toml", feature = "json")), macro_use)]
extern crate indoc;

use page::Page;

pub use data::Data;
pub use errors::{ApiError, Error, Result};
pub use isolang::Language;
pub use mastodon::{Mastodon, MastodonUnauthenticated};
// pub use mastodon_client::{MastodonClient, MastodonUnauthenticated};
pub use mastodon_async_entities::{
    status::NewStatus, status::NewStatusBuilder, visibility::Visibility,
};
pub use registration::Registration;
pub use requests::{AddFilterRequest, AddPushRequest, StatusesRequest, UpdatePushRequest};

/// Registering your App
pub mod apps;
/// Contains the struct that holds the client auth data
pub mod data;
/// Entities returned from the API
pub mod entities;
/// Errors
pub mod errors;
/// Event stream generators
pub mod event_stream;
/// Collection of helpers for serializing/deserializing `Data` objects
pub mod helpers;
/// Handling multiple pages of entities.
pub mod page;
/// Registering your app.
pub mod registration;
/// Requests
pub mod requests;
/// OAuth Scopes
pub mod scopes;

#[macro_use]
mod macros;
/// Automatically import the things you need
pub mod prelude {
    pub use crate::{
        entities::prelude::*, scopes::Scopes, Data, Mastodon, NewStatus, NewStatusBuilder,
        Registration, StatusesRequest, Visibility,
    };
    // Legacy alias; TODO remove for 2.0
    pub use super::entities::status::NewStatusBuilder as StatusBuilder;
}
/// The mastodon client
pub mod mastodon;

/// Legacy aliases. TODO remove for 2.0
pub mod status_builder {
    pub use super::entities::{
        status::{NewStatus, NewStatusBuilder as StatusBuilder},
        visibility::Visibility,
    };
}
pub use status_builder::*;
