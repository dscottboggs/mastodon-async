use crate::error::Result;
use derive_builder::Builder;
use serde::Serialize;

mod bool_qs_serialize {
    use serde::Serializer;

    pub fn is_false(b: &bool) -> bool {
        !*b
    }

    pub fn serialize<S: Serializer>(b: &bool, s: S) -> Result<S::Ok, S::Error> {
        if *b {
            s.serialize_i64(1)
        } else {
            s.serialize_i64(0)
        }
    }
}

#[derive(Debug, Default, Serialize, Builder)]
#[builder(
    derive(Debug, PartialEq),
    build_fn(error = "crate::Error", private, name = "try_build")
)]
pub struct Options {
    #[serde(skip_serializing_if = "bool_qs_serialize::is_false")]
    #[serde(serialize_with = "bool_qs_serialize::serialize")]
    #[builder(default)]
    only_media: bool,
    #[serde(skip_serializing_if = "bool_qs_serialize::is_false")]
    #[serde(serialize_with = "bool_qs_serialize::serialize")]
    #[builder(default)]
    exclude_replies: bool,
    #[serde(skip_serializing_if = "bool_qs_serialize::is_false")]
    #[serde(serialize_with = "bool_qs_serialize::serialize")]
    #[builder(default)]
    pinned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    max_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    since_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    min_id: Option<String>,
    #[serde(skip_serializing_if = "bool_qs_serialize::is_false")]
    #[serde(serialize_with = "bool_qs_serialize::serialize")]
    #[builder(default)]
    exclude_reblogs: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    tagged: Option<String>,
}

impl Options {
    pub fn to_query_string(&self) -> Result<String> {
        Ok(format!("&{}", serde_qs::to_string(self)?))
    }
    pub fn builder() -> OptionsBuilder {
        OptionsBuilder::default()
    }
}

impl OptionsBuilder {
    pub fn build(&self) -> Options {
        self.try_build().expect("no options are required")
    }
}
