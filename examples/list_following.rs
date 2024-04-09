#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;

use mastodon_async::Result;

#[cfg(feature = "toml")]
async fn run() -> Result<()> {
    use futures_util::StreamExt;

    let mastodon = register::get_mastodon_data().await?;
    let you = mastodon.verify_credentials().await?;

    mastodon
        .following(you.id)
        .await?
        .items_iter()
        .for_each(|acct| async move {
            match acct.acct.chars().filter(|c| *c == '@').count() {
                0 => println!("@{}@tams.tech", acct.username),
                1 => println!("@{}", acct.acct),
                other => panic!("found {other} '@' characters in account name {}", acct.acct),
            };
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
