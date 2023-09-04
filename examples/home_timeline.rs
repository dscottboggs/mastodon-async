#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;

use futures_util::StreamExt;
use mastodon_async::Result;
use std::fs::File;
use tracing_subscriber::{fmt, EnvFilter};

#[cfg(feature = "toml")]
async fn run() -> Result<()> {
    let file = File::create("logfile.txt")?;
    let (non_blocking, _guard) = tracing_appender::non_blocking(file);
    let filter = EnvFilter::default()
        .add_directive("hyper=info".parse().unwrap())
        .add_directive("reqwest=info".parse().unwrap())
        .add_directive("mastodon_async=trace".parse().unwrap());
    fmt()
        .with_env_filter(filter)
        .with_writer(non_blocking)
        .json()
        .init();

    register::get_mastodon_data()
        .await?
        .get_home_timeline()
        .await?
        .items_iter()
        .take(4)
        .for_each(|status| async move {
            print!(
                "\ttoot from {}:\n{}",
                status.account.display_name,
                html2text::parse(status.content.as_bytes())
                    .render_plain(90)
                    .into_string()
            )
        })
        .await;
    Ok(())
}

#[cfg(all(feature = "toml", feature = "mt"))]
#[tokio::main]
async fn main() -> Result<()> {
    run().await
}

#[cfg(all(feature = "toml", not(feature = "mt")))]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    run().await
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!(
        "examples require the `toml` feature, run this command for this example:\n\ncargo run \
         --example print_your_profile --features toml\n"
    );
}
