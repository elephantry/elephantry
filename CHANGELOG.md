# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added
- Impl `From<u64>` for `Interval`;
- Impl `From<std::time::Duration>` for `Interval`;

## [3.3.0] - 2023-05-28

### Added
- New `inspect` feature;
- Adds `ltree`, `lquery` and `ltxtquery` types;
- Impl `Copy` for `inspect::Type`;
- The `Composite` proc macro now supports unnamed struct.

### Changed
- `Box::new` takes references;

### Fixed
- Bit type inspection.

## [3.2.0] - 2023-03-17

### Added
- #[must_use] attribute;
- Impl `Clone`, `Copy`, `Debug`, `Eq` and `PatialEq` for `transaction::*` enums;
- bitflags 2.0.

## [3.1.1] - 2023-02-14

### Fixed
- [Fix c_char type error (i8 vs u8) when building on linux arm64](https://github.com/elephantry/elephantry/pull/25)

### Changed
- Removes `inspect::Relation::ty` field;
- Removes `inspect::Domain::constraint` field;

## [3.1.0] - 2023-02-11
### Added
- `pg14` feature;
- Impl `Eq` for `Bytea`, `Jsonb` and `Hstore`;
- Impl `FromSql`/`ToSql` for `[u8; N]`;
- Impl `TryFrom<elephantry::Interval>` for `std::time::Duration`;
- Impl `TryFrom<elephantry::Interval>` for `chrono::Duration`;
- New `FromText`/`ToText` to easily convert type from/to text.
- Major inspector improvemnts:
    - Inspect constraints and indexes;
    - Inspect extensions;
    - Impl `Clone` for `inspect::*` types;
    - New `inspect::Relation::kind` field;
    - New `inspect::Relation::persistence` field;
    - New `inspect::Domain::constraints` field.

### Changed
- `inspect::Relation::ty` is deprecated;
- `inspect::Domain::constraint` is deprecated.

### Fixed
- `u8` conversion.

## [3.0.0] - 2022-04-27
### Added
- Multirange support (pg >= 14);
- Arbitrary crate support;
- Makes pq::Result thread safe;
- `Connection::copy`.

### Changed
- `Entity` trait is no longer automatically impl for type impl `FromSql` +
    `ToSql`. You need to impl the `entity::Simple` empty trait. As a
    counterpart, you can use nested entity, see the `07-relation` example;
- Better derive error;
- Removes Model lifetime.
- libpq 3.0;
- config 0.13;
- ipnetwork 0.19;
- time 0.3;
- uuid 1.0.

### Removed
-  `Composite` and `Enum` traits, in flavor of implementing `FromSql`/`ToSql`
    traits directly.

### Fixed
- Update query without field.

## [2.1.0] - 2021-04-15
### Added
- Derive proc_macro can generates structure and model;
- Impl bit asign for Where.

## [2.0.0] - 2021-03-19
### Added
- `Config` supports all parameters.

### Changed
- Removes deprecated code;

## [1.7.0] - 2021-03-18
### Added
- `all-types` feature;
- [config](https://crates.io/crates/config) support via the
    `config-support` feature;
- `elephantry::Connection::ping` function;
- Derive macros support generic.

### Deprecated
- `serde-support` feature in favour of `serde`.

## [1.6.0] - 2021-02-19
### Added
- Checks features at compilation time for `Entity` derive.
- Cli displays error if schema/relation doesn’t exist.

### Fixed
- Cli correctly handles array types;
- cli prefix reserved rust keyword;
- Cli generates mod.rs files.

### Deprecated
- `elephantry::inspect` functions will returns `crate::Result` in next major
    version.


## [1.5.0] - 2121-02-02
### Added
- Impl `Clone` for `Connection` and `Pool`.

## [1.4.0] - 2121-01-22
### Added
- `values` macro;
- `Rows::into_vec` function.

## [1.3.0] - 2020-12-03
### Added
- Impl `Default` for `where::Builder`;
- `Connection::upset` function;
- `Connection::unlisten` function.

## [1.2.0] - 2020-12-02
### Added
- `Where` and `where::Builder` structs to dynamically create where clause;
- Impl FromSql/ToSql for `()`.
- `Pager::new()` is now public;
- New `Pager::rows()` function;
- `Pager` now implements `IntoIterator`.

## [1.1.1] - 2020-11-15
### Changed
- `Option::None` is now cast as Unknow type in SQL.

### Fixed
- Correctly type array in SQL.

## [1.1.0] - 2020-11-15
### Fixed
- Empty vec to sql convertion.

## [1.0.0] - 2020-10-05

First stable release.
