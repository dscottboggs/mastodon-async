#![deny(warnings)]
use serde::Deserialize;
use serde::Serialize;

/// Error types for this crate
pub mod error;
pub use error::Error;

/// Data structures for ser/de of account-related resources
pub mod account;
/// Data structures for ser/de of admin-related resources
pub mod admin;
/// Data structures for ser/de of announcement-related resources
pub mod announcement;
/// Data structures for ser/de of application-related resources
pub mod application;
/// Data structures for ser/de of attachment-related resources
pub mod attachment;
/// Data structures for ser/de of auth-related resources
pub mod auth;
/// Data structures for ser/de of card-related resources
pub mod card;
/// Data structures for ser/de of context-related resources
pub mod context;
/// Data structures for ser/de of conversation-related resources
pub mod conversation;
/// Module for converting values while serializing and deserializing.
mod conversion;
/// Data structures for ser/de of custom emoji
pub mod custom_emoji;
/// Data structures for ser/de of streaming events
pub mod event;
/// Data structures for ser/de of filter-related resources
pub mod filter;
/// Builders for form submissions
pub mod forms;
/// Type-safe ID values
pub mod ids;
pub use ids::*;
/// Data structures for ser/de of instance-related resources
pub mod instance;
/// Data structures for ser/de of list-related resources
pub mod list;
/// Represents the last read position within a user's timelines.
pub mod marker;
/// Data structures for ser/de of mention-related resources
pub mod mention;
/// Data structures for ser/de of notification-related resources
pub mod notification;
/// Data structures for working with user preferences.
pub mod preferences;
/// Data structures for ser/de of push-subscription-related resources
pub mod push;
/// Data structures for ser/de of relationship-related resources
pub mod relationship;
/// Data structures for ser/de of report-related resources
pub mod report;
/// Data structures for ser/de of search-related resources
pub mod search_result;
/// Data structures for ser/de of status-related resources
pub mod status;
/// Data structures for ser/de of tags.
pub mod tag;
mod test;
/// Data structure for ser/de visibility
pub mod visibility;

/// An empty JSON object.
#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Empty {}

/// The purpose of this module is to alleviate imports of many common
/// structs by adding a glob import to the top of mastodon heavy
/// modules:
pub mod prelude {
    pub use super::{
        account::{
            self, /* for
                  SuggestionSource, Suggestion, FamiliarFollowers, Color, Credentials,
                  CredentialsBuilder */
            Account, CredentialAccount, Role, RolePermissions, Source,
        },
        admin::prelude::*,
        announcement::{self /* for Status, Account, Reaction */, Announcement},
        application::Application,
        attachment::{
            self, /* for FocalPoint, SizeSpecificDetails, Meta */
            Attachment, MediaType, ProcessedAttachment,
        },
        auth::prelude::*,
        card::{self /* for Type */, Card, TrendsLink},
        context::Context,
        conversation::Conversation,
        custom_emoji::CustomEmoji,
        event::Event,
        filter::{self /* for Action, Keyword, Status, v1, Result */, Filter, FilterContext},
        forms,
        ids::*,
        instance::{
            self, /* for
                  Usage, Users, Thumbnail, ThumbnailVersions, Contact, Registrations,
                  Rule, Activity, Configuration, ExtendedDescription */
            DomainBlock, Instance,
        },
        list::{self /* for RepliesPolicy */, List},
        marker::Marker,
        mention::Mention,
        notification::{self /* for Type */, Notification},
        preferences::Preferences,
        push::{
            self, /* for Alerts, AdminAlerts, add_subscription, update_data */
            Subscription,
        },
        relationship::Relationship,
        report::{self /* for Category */, Report},
        search_result::SearchResult,
        status::{
            self, /* for Scheduled, Source, Tag, Application, FeaturedTag, Mention*/
            NewStatus, NewStatusBuilder, Poll, PollBuilder, Status,
        },
        tag::{self /* for History */, Tag},
        visibility::Visibility,
        Empty,
    };
}
