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

### Use Rustls instead of OpenSSL

To use Rustls instead of OpenSSL for HTTPS request, define the dependency as follows

```toml
mastodon-async = { version = "1", default-features = false, features = ["rustls-tls"] }
```

## A Note on Debugging
This library offers structured logging, through the `tracing` crate. See the
[`home_timeline` example](examples/home_timeline.rs) for an example of usage.

This feature is still an unstable part of the dependant crates. This means that
unless `--cfg tracing_unstable` is passed to `rustc`, you'll see an error like
this:

```
error[E0277]: the trait bound `valuable::Value<'_>: tracing::Value` is not satisfied
 --> src/main.rs:5:5
  |
5 | /     debug!(
6 | |         example = valuable::Valuable::as_value(&tracing_value::Example::from("Hello, world")),
7 | |         "test"
8 | |     );
  | |_____^ the trait `tracing::Value` is not implemented for `valuable::Value<'_>`
  |
  = help: the following other types implement trait `tracing::Value`:
...
```

Until tracing [stabilizes this feature](https://docs.rs/tracing/latest/tracing/index.html#unstable-features),
it's necessary to either run cargo commands with `RUSTFLAGS="--cfg tracing_unstable"`,
or to create a file at `.cargo/config` relative to the root of your dependent
crate with the following contents:

```toml
[build]
rustflags = ["--cfg", "tracing_unstable"]
```

See also #117.

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

