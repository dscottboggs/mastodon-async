use std::io::{self, BufRead, Write};

use crate::{errors::Result, registration::Registered, Mastodon};

/// Finishes the authentication process for the given `Registered` object,
/// using the command-line
pub async fn authenticate(registration: Registered) -> Result<Mastodon> {
    let url = registration.authorize_url()?;

    let code = {
        let stdout = io::stdout();
        let stdin = io::stdin();

        let mut stdout = stdout.lock();
        let mut stdin = stdin.lock();

        writeln!(&mut stdout, "Click this link to authorize: {}", url)?;
        write!(&mut stdout, "Paste the returned authorization code: ")?;
        stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;
        input
    };
    let code = code.trim();

    registration.complete(code).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_sync() {
        fn assert_send_sync(_: impl Send + Sync) {}

        let mock_reg = || -> Registered { unimplemented!() };
        let no_run = || async move {
            let _ = authenticate(mock_reg()).await;
        };
        assert_send_sync(no_run());
    }
}
