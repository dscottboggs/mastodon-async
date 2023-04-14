/// Tests for the API surface in terms of what pathes are available
///
/// These are only compile-tests. Nothing is run here.

#[test]
#[allow(unused_imports)]
#[allow(dead_code)]
fn test_compile_imports() {
    use mastodon_async::apps::App as _;
    use mastodon_async::apps::AppBuilder as _;
    use mastodon_async::data::Data as _;
    use mastodon_async::entities::account::Account as _;
    use mastodon_async::entities::account::MetadataField as _;
    use mastodon_async::entities::account::Source as _;
    use mastodon_async::entities::attachment::Attachment as _;
    use mastodon_async::entities::attachment::ImageDetails as _;
    use mastodon_async::entities::attachment::MediaType as _;
    use mastodon_async::entities::attachment::Meta as _;
    use mastodon_async::entities::card::Card as _;
    use mastodon_async::entities::context::Context as _;
    use mastodon_async::entities::event::Event as _;
    use mastodon_async::entities::filter::Filter as _;
    use mastodon_async::entities::filter::FilterContext;
    use mastodon_async::entities::instance::Instance as _;
    use mastodon_async::entities::instance::Stats as _;
    use mastodon_async::entities::instance::StreamingApi as _;
    use mastodon_async::entities::list::List as _;
    use mastodon_async::entities::mention::Mention as _;
    use mastodon_async::entities::notification::Notification as _;
    use mastodon_async::entities::notification::NotificationType as _;
    use mastodon_async::entities::notification::NotificationType as _;
    use mastodon_async::entities::push::Alerts as _;
    use mastodon_async::entities::push::Subscription as _;
    use mastodon_async::entities::relationship::Relationship as _;
    use mastodon_async::entities::report::Report as _;
    use mastodon_async::entities::search_result::SearchResult as _;
    use mastodon_async::entities::status::Application as _;
    use mastodon_async::entities::status::Emoji as _;
    use mastodon_async::entities::status::Mention as _;
    use mastodon_async::entities::status::Status as _;
    use mastodon_async::entities::status::Tag as _;
    use mastodon_async::entities::status::TagHistory as _;
    use mastodon_async::entities::Empty as _;
    use mastodon_async::errors::ApiError as _;
    use mastodon_async::errors::Error as _;
    use mastodon_async::errors::Result as _;
    use mastodon_async::event_stream::event_stream as _;
    use mastodon_async::format_err as _;
    use mastodon_async::helpers::cli::authenticate as _;
    use mastodon_async::helpers::read_response::read_response as _;
    // use mastodon_async::response as _;
    use mastodon_async::mastodon::Mastodon as _;
    use mastodon_async::mastodon::MastodonClient as _;
    use mastodon_async::mastodon::MastodonUnauthenticated as _;
    use mastodon_async::page::Page as _;
    use mastodon_async::registration::Registered as _;
    use mastodon_async::registration::Registration as _;
    use mastodon_async::requests::AddFilterRequest as _;
    use mastodon_async::requests::AddPushRequest as _;
    use mastodon_async::requests::Keys as _;
    use mastodon_async::requests::StatusesRequest as _;
    use mastodon_async::requests::UpdateCredsRequest as _;
    use mastodon_async::requests::UpdatePushRequest as _;
    use mastodon_async::scopes::Read as _;
    use mastodon_async::scopes::Scopes as _;
    use mastodon_async::scopes::Write as _;
    use mastodon_async::status_builder::NewStatus as _;
    use mastodon_async::status_builder::StatusBuilder as _;
    use mastodon_async::status_builder::Visibility as _;
    use mastodon_async::AddFilterRequest;
    use mastodon_async::AddPushRequest as _;
    use mastodon_async::ApiError as _;
    use mastodon_async::Data as _;
    use mastodon_async::Error as _;
    use mastodon_async::Language as _;
    use mastodon_async::Mastodon as _;
    use mastodon_async::MastodonUnauthenticated as _;
    use mastodon_async::NewStatus as _;
    use mastodon_async::Registration as _;
    use mastodon_async::Result as _;
    use mastodon_async::StatusBuilder as _;
    use mastodon_async::StatusesRequest as _;
    use mastodon_async::UpdateCredsRequest as _;
    use mastodon_async::UpdatePushRequest as _;
    use mastodon_async::Visibility as _;
}

#[test]
#[allow(unused_imports)]
#[allow(dead_code)]
fn test_use_prelude() {
    use mastodon_async::entities::prelude::Account as _;
    use mastodon_async::entities::prelude::Application as _;
    use mastodon_async::entities::prelude::Attachment as _;
    use mastodon_async::entities::prelude::Card as _;
    use mastodon_async::entities::prelude::Context as _;
    use mastodon_async::entities::prelude::Emoji as _;
    use mastodon_async::entities::prelude::Empty as _;
    use mastodon_async::entities::prelude::Event as _;
    use mastodon_async::entities::prelude::Filter as _;
    use mastodon_async::entities::prelude::FilterContext as _;
    use mastodon_async::entities::prelude::Instance as _;
    use mastodon_async::entities::prelude::List as _;
    use mastodon_async::entities::prelude::MediaType as _;
    use mastodon_async::entities::prelude::Mention as _;
    use mastodon_async::entities::prelude::Notification as _;
    use mastodon_async::entities::prelude::Relationship as _;
    use mastodon_async::entities::prelude::Report as _;
    use mastodon_async::entities::prelude::SearchResult as _;
    use mastodon_async::entities::prelude::Source as _;
    use mastodon_async::entities::prelude::Stats as _;
    use mastodon_async::entities::prelude::Status as _;
    use mastodon_async::entities::prelude::StreamingApi as _;
    use mastodon_async::entities::prelude::Subscription as _;
    use mastodon_async::prelude::Data as _;
    use mastodon_async::prelude::Mastodon as _;
    use mastodon_async::prelude::NewStatus as _;
    use mastodon_async::prelude::Registration as _;
    use mastodon_async::prelude::Scopes as _;
    use mastodon_async::prelude::StatusBuilder as _;
    use mastodon_async::prelude::StatusesRequest as _;
    use mastodon_async::prelude::Visibility as _;
}

#[cfg(feature = "env")]
#[test]
#[allow(unused_imports)]
#[allow(dead_code)]
fn test_env_exports() {
    use mastodon_async::helpers::env::from_env as _;
    use mastodon_async::helpers::env::from_env_prefixed as _;
}

#[cfg(feature = "json")]
#[test]
#[allow(unused_imports)]
#[allow(dead_code)]
fn test_json_exports() {
    use mastodon_async::helpers::json::from_file as _;
    use mastodon_async::helpers::json::from_reader as _;
    use mastodon_async::helpers::json::from_slice as _;
    use mastodon_async::helpers::json::from_str as _;
    use mastodon_async::helpers::json::to_file as _;
    use mastodon_async::helpers::json::to_file_with_options as _;
    use mastodon_async::helpers::json::to_string as _;
    use mastodon_async::helpers::json::to_vec as _;
    use mastodon_async::helpers::json::to_writer as _;
}

#[cfg(feature = "toml")]
#[test]
#[allow(unused_imports)]
#[allow(dead_code)]
fn test_toml_exports() {
    use mastodon_async::helpers::toml::from_file as _;
    use mastodon_async::helpers::toml::from_reader as _;
    use mastodon_async::helpers::toml::from_slice as _;
    use mastodon_async::helpers::toml::from_str as _;
    use mastodon_async::helpers::toml::to_file as _;
    use mastodon_async::helpers::toml::to_file_with_options as _;
    use mastodon_async::helpers::toml::to_string as _;
    use mastodon_async::helpers::toml::to_vec as _;
    use mastodon_async::helpers::toml::to_writer as _;
}
