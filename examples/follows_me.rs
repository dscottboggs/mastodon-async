#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;
mod tracing;
use mastodon_async::Result;

#[cfg(feature = "toml")]
async fn run() -> Result<()> {
    use futures::StreamExt;

    let _guard = crate::tracing::init_default()?;

    let mastodon = register::get_mastodon_data().await?;
    mastodon
        .follows_me()
        .await?
        .items_iter()
        .for_each(|account| async move {
            println!("{}", account.acct);
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
         --example follows_me --features toml\n"
    );
}
