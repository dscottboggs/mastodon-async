#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]

pub use elefren::prelude::*;

use std::io;

#[cfg(feature = "toml")]
use elefren::helpers::toml;
use elefren::{helpers::cli, Result};

#[allow(dead_code)]
#[cfg(feature = "toml")]
#[tokio::main]
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
pub fn read_line(message: &str) -> Result<String> {
    println!("{}", message);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

#[cfg(not(feature = "toml"))]
fn main() {}
