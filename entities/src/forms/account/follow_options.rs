use crate::forms::helpers::bool_qs;
use derive_builder::Builder;
use isolang::Language;
use serde::Serialize;

#[derive(Debug, Default, Serialize, Clone, Builder)]
#[builder(
    custom_constructor,
    derive(Debug, PartialEq),
    build_fn(error = "crate::Error", private, name = "try_build")
)]
pub struct FollowOptions {
    #[builder(default = "true")]
    #[serde(skip_serializing_if = "bool_qs::is_true")]
    #[serde(serialize_with = "bool_qs::serialize")]
    reblogs: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "bool_qs::is_false")]
    #[serde(serialize_with = "bool_qs::serialize")]
    notify: bool,
    #[builder(default, setter(into))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    languages: Vec<Language>,
}

impl FollowOptions {
    pub fn builder() -> FollowOptionsBuilder {
        FollowOptionsBuilder::create_empty()
    }
}

impl FollowOptionsBuilder {
    pub fn build(&self) -> FollowOptions {
        self.try_build().expect("there were no required options")
    }
    pub fn language(&mut self, language: Language) -> &mut Self {
        self.languages
            .get_or_insert_with(Default::default)
            .push(language);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_serialization() {
        let opt = FollowOptions::builder().build();
        assert_eq!(serde_qs::to_string(&opt).unwrap(), "");
        let opt = FollowOptions::builder().notify(true).build();
        assert_eq!(serde_qs::to_string(&opt).unwrap(), "notify=1");
        let opt = FollowOptions::builder().language(Language::Eng).build();
        // Mastodon doesn't document support for ISO-639-3 (only ISO-639-1; i.e.
        // "two-letter language codes"), but nonetheless support is there for
        // both.
        assert_eq!(serde_qs::to_string(&opt).unwrap(), "languages[0]=eng");
        let opt = FollowOptions::builder()
            .notify(true)
            .reblogs(false)
            .language(Language::Eng)
            .language(Language::Eso)
            .build();
        assert_eq!(
            serde_qs::to_string(&opt).unwrap(),
            "reblogs=0&notify=1&languages[0]=eng&languages[1]=eso"
        );
    }
}
