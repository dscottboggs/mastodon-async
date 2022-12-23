#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;
use mastodon_async::{Result, StatusBuilder, Visibility};

#[cfg(feature = "toml")]
fn bool_input(message: impl AsRef<str>, default: bool) -> Result<bool> {
    let input = register::read_line(message.as_ref())?;
    if let Some(first_char) = input.chars().next() {
        match first_char {
            'Y' | 'y' => Ok(true),
            'N' | 'n' => Ok(false),
            '\n' => Ok(default),
            _ => {
                print!(
                    "I didn't understand '{input}'. Please input something that begins with 'y' \
                     or 'n', case insensitive:  "
                );
                bool_input(message, default)
            },
        }
    } else {
        Ok(default)
    }
}

#[cfg(feature = "toml")]
#[tokio::main]
async fn main() -> Result<()> {
    femme::with_level(femme::LevelFilter::Trace);
    let mastodon = register::get_mastodon_data().await?;
    let input = register::read_line("Enter the path to the photo you'd like to post: ")?;

    let media = mastodon.media(input).await?;
    let status = StatusBuilder::new()
        .status("Mastodon-async photo upload example/demo (automated post)")
        .media_ids([media.id])
        .visibility(Visibility::Private)
        .build()?;
    let status = mastodon.new_status(status).await?;
    println!("successfully uploaded status. It has the ID {}.", status.id);
    if bool_input("would you like to delete the post now? (Y/n)  ", true)? {
        mastodon.delete_status(&status.id).await?;
        println!("ok. done.");
    }

    Ok(())
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!(
        "examples require the `toml` feature, run this command for this example:\n\ncargo run \
         --example upload_photo --features toml\n"
    );
}
