# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2022-xx-xx
### Added

### Changed


## [v3.0.1] - 2022-05-31
### Added
- Support for custom access denied response #26

## [v3.0.0] - 2022-04-03
### Added
- `actix-web: 4.0.1` support #30

## [v3.0.0-beta.6] - 2022-01-08
### Added
- Support custom types for permissions/roles #25
- `actix-web: 4.0.0-beta.19` support #27

## [v3.0.0-beta.5] - 2021-12-30
### Added
- `actix-web: 4.0.0-beta.18` support #24

## [v3.0.0-beta.4] - 2021-11-17
### Added
- `actix-web: 4.0.0-beta.11` support #22
### Changed
- Remove unused `Config` type #20

## [v3.0.0-beta.3] - 2021-10-11
### Added
- Allow extractor to receive mutable requests #17
- How to use section to readme #18

## [v2.2.0] - 2021-10-11
### Added
- Allow extractor to receive mutable requests #17
- How to use section to readme #18

## [v3.0.0-beta.2] - 2021-08-11
### Changed
- Pull incoming changes from [v2.1.0]
- Update `actix-web` to `4.0.0-beta.8`
- Remove RefCell in middleware #11

## [v2.1.0] - 2021-08-11
### Added
- Add support for ABAC-like model to procedural macro [#14](https://github.com/DDtKey/actix-web-grants/issues/14)

### Changed
- Change Arc to Rc in middleware #12

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
- Breaking change(!): remove Arc usage in `PermissionsExtractor` #1

## [v0.1.6] - 2021-01-18
### Changed
- Remove extra and insecure dependencies


[v0.1.6]: https://crates.io/crates/actix-web-grants/0.1.6
[v1.0.0]: https://crates.io/crates/actix-web-grants/1.0.0
[v2.0.0]: https://crates.io/crates/actix-web-grants/2.0.0
[v2.0.1]: https://crates.io/crates/actix-web-grants/2.0.1
[v2.1.0]: https://crates.io/crates/actix-web-grants/2.1.0
[v2.2.0]: https://crates.io/crates/actix-web-grants/2.2.0
[v2.3.0]: https://crates.io/crates/actix-web-grants/2.3.0
[v3.0.0-beta.1]: https://crates.io/crates/actix-web-grants/3.0.0-beta.1
[v3.0.0-beta.2]: https://crates.io/crates/actix-web-grants/3.0.0-beta.2
[v3.0.0-beta.3]: https://crates.io/crates/actix-web-grants/3.0.0-beta.3
[v3.0.0-beta.4]: https://crates.io/crates/actix-web-grants/3.0.0-beta.4
[v3.0.0-beta.5]: https://crates.io/crates/actix-web-grants/3.0.0-beta.5
[v3.0.0-beta.6]: https://crates.io/crates/actix-web-grants/3.0.0-beta.6
[v3.0.0]: https://crates.io/crates/actix-web-grants/3.0.0
[v3.0.1]: https://crates.io/crates/actix-web-grants/3.0.1
