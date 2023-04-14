#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;
mod tracing;

use mastodon_async::Result;

#[cfg(feature = "toml")]
async fn run() -> Result<()> {
    let _guard = crate::tracing::init_json()?;

    let mastodon = register::get_mastodon_data().await?;
    let input = register::read_line("Enter the term you'd like to search: ")?;
    let result = mastodon.search(&input, false).await?;

    println!("{:#?}", result);

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
         --example search --features toml\n"
    );
}
