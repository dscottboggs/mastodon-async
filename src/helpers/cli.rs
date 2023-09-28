use std::io::{self, BufRead, Write};

use mastodon_async_entities::{forms, ids::OAuthToken};

use crate::errors::Result;

/// Ask the user to get you an authorized OAuth token
pub async fn get_oauth_token(
    authorization_request: forms::oauth::AuthorizationRequest,
) -> Result<OAuthToken> {
    let url = authorization_request.url()?;

    let code = {
        let stdout = io::stdout();
        let stdin = io::stdin();

        let mut stdout = stdout.lock();
        let mut stdin = stdin.lock();

        writeln!(&mut stdout, "Click this link to authorize: {url}")?;
        write!(&mut stdout, "Paste the returned authorization code: ")?;
        stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;
        input
    };
    Ok(code.trim().to_owned().into())
}
