#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;
use mastodon_async::Result;

#[cfg(feature = "toml")]
async fn run() -> Result<()> {
    use futures_util::StreamExt;
    use mastodon_async::StatusesRequest;

    let mut filters = StatusesRequest::new();
    filters.limit(3);
    let mastodon = register::get_mastodon_data().await?;
    let you = mastodon.verify_credentials().await?;

    mastodon
        .statuses(&you.id, filters)
        .await?
        .items_iter()
        .take(4)
        .for_each(|status| async move { println!("{status:?}") })
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
         --example get_statuses --features toml\n"
    );
}
