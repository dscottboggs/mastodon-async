#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;
use mastodon_async::Result;

#[cfg(feature = "toml")]
#[tokio::main]
async fn main() -> Result<()> {
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
        .for_each(|status| async move { println!("{status:?}") })
        .await;
    Ok(())
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!(
        "examples require the `toml` feature, run this command for this example:\n\ncargo run \
         --example get_statuses --features toml\n"
    );
}
