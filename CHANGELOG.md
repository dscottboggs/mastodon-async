<a name="v0.18.0"></a>
## v0.18.0 (2019-01-05)

#### Breaking Changes

* Change `Registered::complete` to take the receiver by reference

#### Features

* Add `Registered::into_parts` and `Registered::from_parts`
* Add `FromStr` implementation for `Scopes`

<a name="v0.17.0"></a>
## v0.17.0 (2018-12-12)

#### Bug Fixes

* Fixes `follow` to return a `Relationship` [6f63fef](6f63fef0b2414099c55e36107aab785f689d19b6)

#### Features

* Add `follows_me` and `followed_by_me` shortcut methods [4dd4042](4dd40422b3de11aaadcfc600317c0df3556b38f5)

#### Breaking Changes

* Change `u64` ids to `&str` [9a08058](9a080583f0577d3f77eaf7be55758835c56ea61e)
* Fixes `follow` to return a `Relationship` [6f63fef](6f63fef0b2414099c55e36107aab785f689d19b6)

<a name="v0.16.0"></a>
## v0.16.0 (2018-10-03)

This release upgrades reqwest, our http client dependency, from 0.8.8 ->
0.9.0

<a name="v0.15.2"></a>
## v0.15.2

This is a bugfix release

<a name="v0.15.1"></a>
## v0.15.1

This is a bugfix release

<a name="v0.15.0"></a>
## v0.15.0 (2018-09-15)

This release adds support for all new API endpoints introduced in Mastodon 2.4.* and 2.5.0

#### Features

* **scopes:** Implement granular OAuth scopes ([e284894d](e284894d), closes [#44](44))
* **helpers:** cli::authenticate ([034bd4e6](034bd4e6))
* **client:** Implement client.update\_credentials ([a57c7e2f](a57c7e2f))
* **status:** Add language code to status builer ([989d9a59](989d9a59), closes [#55](55))
* **client:** Implement profile metadata update ([0ad1e374](0ad1e374), closes [#54](54))
* **search:** Implement `GET /api/v2/search` ([28192e11](28192e11))
* **client:** Implement push notifciations endpoints ([690b029d](690b029d), closes [#53](53))
* **client:** Add `replies_count` property to `Status` entities ([7d752a9f](7d752a9f), closes [#73](73))
* **client:** Implement Keyword filtering API ([7d164cb8](7d164cb8), closes [#71](71))
* **client:** Implement the Follow Suggestions API ([7de1bdc0](7de1bdc0), closes [#72](72))
* **client:** Implement the Endorsements API ([1f0ba184](1f0ba184), closes [#74](74))

#### Breaking Changes

* **scopes:** Implement granular OAuth scopes ([e284894d](e284894d)) (closes [#44](44))

#### Bug Fixes

* **client:** change return value of client.unfollow to `Relationship` ([20a9c69a](20a9c69a))
* **client:** change `POST /search` to `GET /search` ([34e2c008](34e2c008))


<a name="v0.14.0"></a>
## v0.14.0 (2018-08-29)

#### Features

* **helpers:**  add json helper module ([46871da4](46871da4))

#### Breaking Changes

* **client:**  fix broken paged\_routes\_with\_id methods ([c66c305d](c66c305d), closes [#42](42))

#### Bug Fixes

* **client:**  fix broken paged\_routes\_with\_id methods ([c66c305d](c66c305d), closes [#42](42))


# 0.13 (2018/08/27)

## Features

- `Registration` now duplicates the `AppBuilder` API, so you can
  replace:

  ```
  let app = App::builder();
  app.client_name("test-client");

  let registration = Registration::new("http://example.com")
                                  .register(app)?;
  ```

  with this:

  ```
  let registration = Registration::new("http://example.com")
                                  .client_name("test-client")
                                  .build()?;
  ```

  You can still call use the `Registration` & `AppBuilder` APIs like
  before, but any App passed to `.register` will supercede anything app
  config set on the `Registration` object itself.

  In future releases, this will become a hard error.

- `elefren::status_builder::StatusBuilder::new()` now takes anything
  that implements `Display` instead of specifically an owned `String`

## Breaking Changes

- The `elefren::data::toml` module has been moved to
  `elefren::helpers::toml`
- Because of the changes to `Registration`, the `elefren::apps::prelude`
  module has been removed. The types that are still necessary from that
  prelude have been moved to `elefren::prelude`, but
  `elefren::apps::App` will have to be imported separately
- `elefren::entities::account::CredientialsBuilder` has been moved to
  `elefren::entities::account::CredentialsBuilder` (note the spelling
  difference)
- `Registered::complete` now takes a `&str` instead of a `String`

## Compatibility

- `elefren::entities::instance::Instance` now has the `max_toot_chars`
  property, for use with the Pleroma and Glitch-soc APIs

# 0.12 (2018/08/23)

## Features

- `Page::items_iter` added, abstracts over "next_page" logic
- `elefen::prelude` and `elefen::apps::prelude` modules added to more
  easily import all necessary types
- Helpers added to allow serialization & deseriasization of `Data` types
  to `toml`

## Breaking Changes

- Combined all parameters to `mastodon.statuses` (except `id`) to their
  own type, `StatusesRequest`
- All API methods on `Mastodon` type moved into a trait,
  `MastodonClient`, to enable better mocking during tests
- `Mastodon::from_data(Data)` was changed to `Mastodon::from(Data)`
- `AppBuilder` was changed, use `App::builder()` instead
- `Registration` was broken up to enable better registration flow

## Documentation

- All API methods now have doc comments
- All docs were updated for the new breaking changes

## Compatibility

- Login to pleroma instances was fixed

# 0.11
- Added more examples to `examples` directory.
- Fixed `follow` and `unfollow` routes.
- Updated `moved` field to be `Box<Account>`.

# 0.10

- Added the ability to handle paged entities like favourites and such.(Only favourites in prerelease)
- Added optional `source` and `moved` fields to `Account`.
- Added `Source` struct to match with the `Account.source` field.
- Added `CredientialsBuilder` struct for updating profile using
  `verify_credientials`.
- Attachment now handles being sent an empty object, which is converted
  to `None`.
- Added ombed data fields to `Card`.
- Added `version` and `urls` fields to `Instance`.
- Added `id`, `muting_notifications`, and `domain_blocking` to `Relationship`.
- Added `emojis`, `language`, and `pinned` fields to `Status`
- Added `Emoji` struct.
- Added `List` and `Mention` structs(matching routes not added yet).
- Added example that prints your profile.
- Updated dependencies
