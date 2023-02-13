#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]

pub use mastodon_async::prelude::*;

use std::io;

#[cfg(feature = "toml")]
use mastodon_async::helpers::toml;
use mastodon_async::{helpers::cli, Result};

#[allow(dead_code)]
#[cfg(all(feature = "toml", feature = "mt"))]
#[tokio::main]
async fn main() -> Result<()> {
    register().await?;
    Ok(())
}

#[allow(dead_code)]
#[cfg(all(feature = "toml", not(feature = "mt")))]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    register().await?;
    Ok(())
}

#[allow(dead_code)]
#[cfg(feature = "toml")]
pub async fn get_mastodon_data() -> Result<Mastodon> {
    if let Ok(data) = toml::from_file("mastodon-data.toml") {
        Ok(Mastodon::from(data))
    } else {
        register().await
    }
}

#[cfg(feature = "toml")]
pub async fn register() -> Result<Mastodon> {
    let website = read_line("Please enter your mastodon instance url:")?;
    let registration = Registration::new(website.trim())
        .client_name("elefren-examples")
        .scopes(Scopes::all())
        .website("https://github.com/dscottboggs/mastodon-async")
        .build()
        .await?;
    let mastodon = cli::authenticate(registration).await?;

    // Save app data for using on the next run.
    toml::to_file(&mastodon.data, "mastodon-data.toml")?;

    Ok(mastodon)
}

#[cfg(feature = "toml")]
pub fn read_line(message: impl AsRef<str>) -> Result<String> {
    use std::io::Write;

    print!("{}", message.as_ref());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

#[cfg(feature = "toml")]
#[allow(dead_code)]
pub fn bool_input(message: impl AsRef<str>, default: bool) -> Result<bool> {
    let input = read_line(message.as_ref())?;
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
            }
        }
    } else {
        Ok(default)
    }
}

#[cfg(not(feature = "toml"))]
fn main() {}
