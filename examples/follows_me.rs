#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;
use mastodon_async::Result;

#[cfg(feature = "toml")]
#[tokio::main]
async fn main() -> Result<()> {
    use futures::StreamExt;

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

#[cfg(not(feature = "toml"))]
fn main() {
    println!(
        "examples require the `toml` feature, run this command for this example:\n\ncargo run \
         --example print_your_profile --features toml\n"
    );
}
