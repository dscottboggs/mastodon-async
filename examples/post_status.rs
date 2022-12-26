#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;

use mastodon_async::{Language, Result, StatusBuilder, Visibility};

#[cfg(feature = "toml")]
#[tokio::main]
async fn main() -> Result<()> {
    let mastodon = register::get_mastodon_data().await?;
    let status = StatusBuilder::new()
        .status(register::read_line(
            "Enter a status to post privately (enter to send): ",
        )?)
        .visibility(Visibility::Private)
        .language(Language::Eng)
        .build()?;

    let status = mastodon.new_status(status).await?;

    print!("Status posted");
    if let Some(url) = status.url {
        // this is the expected thing to happen
        println!(", visible when logged in at: {}\n", url);
    } else {
        println!(". For some reason, the status URL was not returned from the server.");
        println!("Maybe try here? {}/{}", status.account.url, status.id);
    }
    if register::bool_input("delete this post? (Y/n)", true)? {
        mastodon.delete_status(&status.id).await?;
        println!("ok, done")
    }

    Ok(())
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!(
        "examples require the `toml` feature, run this command for this example:\n\ncargo run \
         --example post_status --features toml\n"
    );
}
