use serde::{Deserialize, Serialize};
use time::{serde::timestamp, OffsetDateTime};

use super::Scopes;

/// Represents an OAuth token used for authenticating with the API and
/// performing actions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Token {
    /// An OAuth token to be used for authorization.
    pub access_token: String,
    /// The OAuth token type. Mastodon uses Bearer tokens.
    pub token_type: String,
    /// The OAuth scopes granted by this token.
    pub scope: Scopes,
    /// When the token was generated.
    #[serde(with = "timestamp")]
    pub created_at: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use crate::auth::scopes::Scope;

    use super::*;

    #[test]
    fn test_deserialize() {
        let example = r#"{
          "access_token": "ZA-Yj3aBD8U8Cm7lKUp-lm9O9BmDgdhHzDeqsY8tlL0",
          "token_type": "Bearer",
          "scope": "read write follow push",
          "created_at": 1573979017
        }"#;
        let subject: Token = serde_json::from_str(example).unwrap();
        assert_eq!(
            subject.access_token,
            "ZA-Yj3aBD8U8Cm7lKUp-lm9O9BmDgdhHzDeqsY8tlL0"
        );
        assert_eq!(subject.token_type, "Bearer");
        assert!(subject.scope.contains(&Scope::Read(None)));
        assert!(subject.scope.contains(&Scope::Write(None)));
        assert!(subject.scope.contains(&Scope::Follow));
        assert!(subject.scope.contains(&Scope::Push));
        assert_eq!(subject.created_at.unix_timestamp(), 1573979017);
    }
}
