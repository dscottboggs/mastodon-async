use crate::forms::helpers::bool_qs;
use derive_builder::Builder;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(
    custom_constructor,
    derive(Debug, PartialEq),
    build_fn(error = "crate::Error", private, name = "try_build")
)]
pub struct Search {
    #[builder(private)]
    q: String,
    #[serde(skip_serializing_if = "bool_qs::is_false")]
    #[builder(default)]
    following: bool,
    #[serde(skip_serializing_if = "bool_qs::is_false")]
    #[builder(default)]
    resolve: bool,
}

impl SearchBuilder {
    pub fn build(&self) -> Search {
        self.try_build().expect("required fields should be set")
    }
}

impl Search {
    pub fn builder(q: impl Into<String>) -> SearchBuilder {
        let mut b = SearchBuilder::create_empty();
        b.q(q.into());
        b
    }
}
