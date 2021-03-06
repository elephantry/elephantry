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
