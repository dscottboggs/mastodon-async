#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;
mod tracing;
use mastodon_async::Result;

#[cfg(feature = "toml")]
async fn run() -> Result<()> {
    let _guard = crate::tracing::init_default()?;

    use mastodon_async::entities::AccountId;
    let mastodon = register::get_mastodon_data().await?;
    let input = register::read_line("Enter the account id you'd like to follow: ")?;
    let account = AccountId::new(input.trim());
    let new_follow = mastodon.follow(&account).await?;

    println!("{:#?}", new_follow);
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
         --example follow_profile --features toml,mt\n"
    );
}
