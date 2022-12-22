# Async Mastodon client library 

[![Build Status](https://github.com/dscottboggs/mastodon-async/actions/workflows/rust.yml/badge.svg)]
[![crates.io](https://img.shields.io/crates/v/mastodon-async.svg)](https://crates.io/crates/mastodon-async)
[![Docs](https://docs.rs/mastodon-async/badge.svg)](https://docs.rs/mastodon-async)
[![MIT/APACHE-2.0](https://img.shields.io/crates/l/mastodon-async.svg)](https://crates.io/crates/mastodon-async)

[Documentation](https://docs.rs/mastodon-async/)

A type-safe, async wrapper around the client [API](https://github.com/tootsuite/documentation/blob/master/docs/Using-the-API/API.md#tag)
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
version = "0.22"
features = ["toml"]
```

```rust,no_run
// src/main.rs

use std::error::Error;

use mastodon_async::prelude::*;
use mastodon_async::helpers::toml; // requires `features = ["toml"]`
use mastodon_async::helpers::cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mastodon = if let Ok(data) = toml::from_file("mastodon-data.toml") {
        Mastodon::from(data)
    } else {
        register()?
    };

    let you = mastodon.verify_credentials().await?;

    println!("{:#?}", you);

    Ok(())
}

fn register() -> Result<Mastodon, Box<dyn Error>> {
    let registration = Registration::new("https://botsin.space")
                                    .client_name("mastodon-async-examples")
                                    .build()?;
    let mastodon = cli::authenticate(registration)?;

    // Save app data for using on the next run.
    toml::to_file(&*mastodon, "mastodon-data.toml")?;

    Ok(mastodon)
}
```

It also supports the [Streaming API](https://docs.joinmastodon.org/api/streaming):

```rust,no_run
use mastodon_async::prelude::*;
use mastodon_async::entities::event::Event;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    let client = Mastodon::from(Data::default());

    client.stream_user()
        .await?
        .try_for_each(|event| {
            match event {
                Event::Update(ref status) => { /* .. */ },
                Event::Notification(ref notification) => { /* .. */ },
                Event::Delete(ref id) => { /* .. */ },
                Event::FiltersChanged => { /* .. */ },
            }
        })
        .await?;
    Ok(())
}
```
