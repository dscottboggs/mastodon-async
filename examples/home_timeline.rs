#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;
use futures_util::StreamExt;
use mastodon_async::Result;

#[cfg(feature = "toml")]
#[tokio::main]
async fn main() -> Result<()> {
    register::get_mastodon_data()
        .await?
        .get_home_timeline()
        .await?
        .items_iter()
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

#[cfg(not(feature = "toml"))]
fn main() {
    println!(
        "examples require the `toml` feature, run this command for this example:\n\ncargo run \
         --example print_your_profile --features toml\n"
    );
}
