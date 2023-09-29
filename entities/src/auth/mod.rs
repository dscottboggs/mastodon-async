/// OAuth Scopes
pub mod scopes;
pub mod token;

pub use scopes::{Scope, Scopes};
use serde::{Deserialize, Serialize};
pub use token::Token;

pub mod prelude {
    pub use super::{scopes, Scope, Scopes, Token};
}

/// The empty object is returned by a request to revoke an OAuth token. This
/// type represents that.
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize)]
pub struct RevocationResponse {}
