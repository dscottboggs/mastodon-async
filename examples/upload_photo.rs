#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;
use mastodon_async::{prelude::*, Result};
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

    use register::bool_input;
    let mastodon = register::get_mastodon_data().await?;
    let input = register::read_line("Enter the path to the photo you'd like to post: ")?;
    let description = register::read_line("describe the media?  ")?;
    let description = if description.trim().is_empty() {
        None
    } else {
        Some(description)
    };

    let media = mastodon.media(input, description).await?;
    let media = mastodon
        .wait_for_processing(media, Default::default())
        .await?;
    println!("media upload available at: {}", media.url);
    let status = StatusBuilder::default()
        .status("Mastodon-async photo upload example/demo (automated post)")
        .media_ids(vec![media.id])
        .visibility(Visibility::Private)
        .build()?;
    let status = mastodon.new_status(status).await?;
    println!("successfully uploaded status. It has the ID {}.", status.id);
    if bool_input("would you like to delete the post now? (Y/n)  ", true)? {
        mastodon.delete_status(&status.id).await?;
        println!("ok. done.");
    }

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
         --example upload_photo --features toml\n"
    );
}
