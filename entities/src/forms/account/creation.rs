use isolang::Language;
use serde::{Deserialize, Serialize};

/// Form to be submitted in order to create a new account.
///
/// See also [the API documentation](https://docs.joinmastodon.org/methods/accounts/#create)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Creation {
    username: String,
    email: String,
    password: String,
    agreement: bool,
    locale: Language,
    reason: Option<String>,
}

impl Creation {
    pub fn new(
        username: String,
        email: String,
        password: String,
        agreement: bool,
        locale: Language,
    ) -> Self {
        Self {
            username,
            email,
            password,
            agreement,
            locale,
            reason: None,
        }
    }
    pub fn reason(&mut self, reason: impl Into<String>) -> &mut Self {
        self.reason = Some(reason.into());
        self
    }
}
