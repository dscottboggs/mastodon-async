# Async Mastodon client library 

[![Build Status](https://github.com/dscottboggs/mastodon-async/actions/workflows/rust.yml/badge.svg)]
[![crates.io](https://img.shields.io/crates/v/mastodon-async.svg)](https://crates.io/crates/mastodon-async)
[![Docs](https://docs.rs/mastodon-async/badge.svg)](https://docs.rs/mastodon-async)
[![MIT/APACHE-2.0](https://img.shields.io/crates/l/mastodon-async.svg)](https://crates.io/crates/mastodon-async)

[Documentation](https://docs.rs/mastodon-async/)

A type-safe, async wrapper around the client [API](https://docs.joinmastodon.org/client/intro/)
for [Mastodon](https://botsin.space/)

## Installation

To add `mastodon-async` to your project, add the following to the
`[dependencies]` section of your `Cargo.toml`

```toml
mastodon-async = "1.0"
```

Alternatively, run the following command:

~~~console
$ cargo add mastodon-async
~~~

## Example

In your `Cargo.toml`, make sure you enable the `toml` feature:

```toml
[dependencies.mastodon-async]
version = "1.0"
features = ["toml"]
```

```rust,no_run
// src/main.rs

use mastodon_async::prelude::*;
use mastodon_async::helpers::toml; // requires `features = ["toml"]`
use mastodon_async::{helpers::cli, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mastodon = if let Ok(data) = toml::from_file("mastodon-data.toml") {
        Mastodon::from(data)
    } else {
        register().await?
    };

    let you = mastodon.verify_credentials().await?;

    println!("{:#?}", you);

    Ok(())
}

async fn register() -> Result<Mastodon> {
    let registration = Registration::new("https://botsin.space")
                                    .client_name("mastodon-async-examples")
                                    .build()
                                    .await?;
    let mastodon = cli::authenticate(registration).await?;

    // Save app data for using on the next run.
    toml::to_file(&mastodon.data, "mastodon-data.toml")?;

    Ok(mastodon)
}
```

It also supports the [Streaming API](https://docs.joinmastodon.org/api/streaming):

```rust,no_run
use mastodon_async::{prelude::*, Result, entities::event::Event};
use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Mastodon::from(Data::default());

    client.stream_user()
        .await?
        .try_for_each(|event| async move {
            match event {
                Event::Update(ref status) => { /* .. */ },
                Event::Notification(ref notification) => { /* .. */ },
                Event::Delete(ref id) => { /* .. */ },
                Event::FiltersChanged => { /* .. */ },
            }
            Ok(())
        })
        .await?;
    Ok(())
}
```
