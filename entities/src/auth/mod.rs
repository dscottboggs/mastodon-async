/// OAuth Scopes
pub mod scopes;
pub mod token;

pub use scopes::{Scope, Scopes};
pub use token::Token;

pub mod prelude {
    pub use super::{scopes, Scope, Scopes, Token};
}
