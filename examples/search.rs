#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;

use mastodon_async::Result;

#[cfg(feature = "toml")]
#[tokio::main]
async fn main() -> Result<()> {
    let mastodon = register::get_mastodon_data().await?;
    let input = register::read_line("Enter the term you'd like to search: ")?;
    let result = mastodon.search(&input, false).await?;

    println!("{:#?}", result);

    Ok(())
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!(
        "examples require the `toml` feature, run this command for this example:\n\ncargo run \
         --example search --features toml\n"
    );
}
