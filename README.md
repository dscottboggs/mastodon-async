# Async Mastodon client library 

[![Build Status](https://github.com/dscottboggs/mastodon-async/actions/workflows/rust.yml/badge.svg)]
[![crates.io](https://img.shields.io/crates/v/mastodon-async.svg)](https://crates.io/crates/mastodon-async)
[![Docs](https://docs.rs/mastodon-async/badge.svg)](https://docs.rs/mastodon-async)
[![MIT/APACHE-2.0](https://img.shields.io/crates/l/mastodon-async.svg)](https://crates.io/crates/mastodon-async)

# V2 WIP

This is a branch tracking version 2 of this library, which will bring the
library current with all documented routes and entities, and come with some
breaking changes. Once the process of "combing" the documentation is complete,
v2 RCs and then a release will be cut from this branch, which will then be
renamed `main`.

V1 is still being used in some capacity by some projects, but new features are
not being developed for it. Please develop new features targeting this branch,
and review the work already done starting at issue #54.

---

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

### Use Rustls instead of OpenSSL

To use Rustls instead of OpenSSL for HTTPS request, define the dependency as follows

```toml
mastodon-async = { version = "1", default-features = false, features = ["rustls-tls"] }
```

## A Note on Debugging
This library offers structured logging. To get better information about bugs or
how something is working, I recommend adding the femme crate as a dependency,
then adding this line to the beginning of your main() function:

```rust,ignore
femme::with_level(log::LevelFilter::Trace);
```

When compiling for the debug target, this offers a mostly-human-readable output
with a lot of details about what's happening. When targeting release, JSON-
structured metadata is offered, which can be filtered and manipulated with
scripts or at the shell with jq.

There are other crates which make use of the log crate's new (unstable) kv
features, this is just the one that works for me for now.

## Example

In your `Cargo.toml`, make sure you enable the `toml` feature:

```toml
[dependencies.mastodon-async]
version = "1.0"
features = ["toml", "mt"]
```

The `"mt"` feature is for tokio multi-threaded. For single threaded, drop the
`"mt"` feature and replace `#[tokio::main]` with
`#[tokio::main(flavor = "current_thread")]`.

<!--
    todo swap ignore with no_run just below here & the next example

test passes locally but not in CI
-->

```rust,ignore
// src/main.rs

use mastodon_async::prelude::*;
use mastodon_async::helpers::toml; // requires `features = ["toml"]`
use mastodon_async::{helpers::cli, Result};

#[tokio::main] // requires `features = ["mt"]
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

> **Note**: this example compiles, but will not run. See the
> [log_events](https://github.com/dscottboggs/mastodon-async/blob/main/examples/log_events.rs)
> example for a more thorough example which does compile and run.

```rust,ignore
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

