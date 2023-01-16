#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;

use futures_util::StreamExt;
use mastodon_async::Result;
use std::{
    io::Write,
    process::{exit, Command, Stdio},
};

#[cfg(feature = "toml")]
async fn run() -> Result<()> {
    use register::bool_input;
    let mastodon = register::get_mastodon_data().await?;

    let peers: Vec<_> = mastodon
        .instance_peers()
        .await?
        .items_iter()
        .collect()
        .await;

    if bool_input(format!("print {} peers?", peers.len()), false)? {
        let mut process = Command::new("less")
            .stdout(Stdio::inherit())
            .stdin(Stdio::piped())
            .spawn()?;
        let mut pipe = process.stdin.take().unwrap();
        for peer in peers {
            pipe.write_all(peer.as_bytes())?;
            pipe.write_all(&[10])?
        }
    }

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
    use std::process::Stdio;

    println!(
        "examples require the `toml` feature, run this command for this example:\n\ncargo run \
         --example show_instance_peers --features toml\n"
    );
}
