# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2021-xx-xx
### Added
### Changed

## [Unreleased] - 2021-08-11
### Changed
- Update `actix-web` to `4.0.0-beta.8`

## [v3.0.0-beta.1] - 2021-04-07
### Changed
- Update `actix-web` to `4.0.0-beta.5`

## [v2.0.1] - 2021-03-10
### Changed
- Fix bug with `Result` return type combined with` proc-macro` way [#3](https://github.com/DDtKey/actix-web-grants/issues/3)
- Update `actix-rt` dev-dependency to `2` 

## [v2.0.0] - 2021-01-22
### Added
- Add example using `actix-web-httpauth` and `jsonwebtoken`

### Changed
- Change crate category to authentication
- Breaking change(!): change `authoritites` to `permissions` everywhere for more clarity

## [v1.0.0] - 2021-01-18
### Added
- Github Actions: Add CI pipeline

### Changed
- Breaking change(!): remove Arc usage in `PermissionsExtractor` [#1](https://github.com/DDtKey/actix-web-grants/pull/1)

## [v0.1.6] - 2021-01-18
### Changed
- Remove extra and insecure dependencies


[v0.1.6]: https://crates.io/crates/actix-web-grants/0.1.6
[v1.0.0]: https://crates.io/crates/actix-web-grants/1.0.0
[v2.0.0]: https://crates.io/crates/actix-web-grants/2.0.0
[v3.0.0-beta.1]: https://crates.io/crates/actix-web-grants/v3.0.0-beta.1
