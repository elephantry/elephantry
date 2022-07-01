# Next

- `pg14` feature
- Impl `Eq` for `Bytea`, `Jsonb` and `Hstore`

# Version 3.0.0

- Adds multirange support (pg >= 14);
- `Entity` trait is no longer automatically impl for type impl `FromSql` +
    `ToSql`. You need to impl the `entity::Simple` empty trait. As a
    counterpart, you can use nested entity, see the `07-relation` example;
- Removes `Composite` and `Enum` traits, in flavor of implementing
    `FromSql`/`ToSql` traits directly;
- Adds arbitrary crate support;
- Makes pq::Result thread safe;
- Fixes update query without field;
- Adds Connection::copy;
- Better derive error;
- Removes Model lifetime.

## Dependencies

- libpq 3.0;
- config 0.13;
- ipnetwork 0.19;
- time 0.3;
- uuid 1.0.

# Version 2.1.0

- Derive proc_macro can generates structure and model;
- Impl bit asign for Where.

# Version 2.0.0

- Removes deprecated code;
- `Config` supports all parameters.

# Version 1.7.0

- Adds `all-types` feature;
- Deprecates `serde-support` feature in favour of `serde`;
- Adds [config](https://crates.io/crates/config) support via the
    `config-support` feature;
- Adds `elephantry::Connection::ping` function;
- Derive macros support generic.

# Version 1.6.0

- `elephantry::inspect` functions will returns `crate::Result` in next major
    version;
- Checks features at compilation time for `Entity` derive.

## Cli

- Display error if schema/relation doesn’t exist;
- Correctly handles array types;
- Prefix reserved rust keyword;
- Generates mod.rs files.

# Version 1.5.0

- Impl `Clone` for `Connection` and `Pool`.

# Version 1.4.0

- Adds `values` macro;
- Adds `Rows::into_vec` function.

# Version 1.3.0

- Impl `Default` for `where::Builder`;
- Adds `Connection::upset` function;
- ADds `Connection::unlisten` function.

# Version 1.2.0

- Adds `Where` and `where::Builder` structs to dynamically create where clause;
- Impl FromSql/ToSql for `()`.

## Pager

- `Pager::new()` is now public;
- New `Pager::rows()` function;
- `Pager` now implements `IntoIterator`.

# Version 1.1.1

- Correctly type array in SQL;
- Option::None is now cast as Unknow type in SQL.

# Version 1.1.0

- Fixes empty vec to sql convertion.

# Version 1.0.0

First stable release.
