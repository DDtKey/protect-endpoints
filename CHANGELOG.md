# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2021-xx-xx
### Added

### Changed

## [v2.2.0] - 2021-10-11
### Added
- Allow extractor to receive mutable requests [#17](https://github.com/DDtKey/actix-web-grants/pull/17)
- Add 'How to use' section to README.md [#18](https://github.com/DDtKey/actix-web-grants/pull/18)

## [v2.1.0] - 2021-08-11
### Added
- Add support for ABAC-like model to procedural macro [#14](https://github.com/DDtKey/actix-web-grants/issues/14)

### Changed
- Change Arc to Rc in middleware [#12](https://github.com/DDtKey/actix-web-grants/pull/12)

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
[v2.1.0]: https://crates.io/crates/actix-web-grants/2.1.0
[v2.2.0]: https://crates.io/crates/actix-web-grants/2.2.0
