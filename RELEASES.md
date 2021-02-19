# Verion 1.6.0

- `elephantry::inspect` functions will returns `crate::Result` in next major
    version;
- Checks features at compilation time for `Entity` derive.

## Cli

- Display error if schema/relation doesn’t exist;
- Correctly handles array types;
- Prefix reserved rust keyword;
- Generates mod.rs files.

# Verion 1.5.0

- Impl `Clone` for `Connection` and `Pool`.

# Verion 1.4.0

- Adds `values` macro;
- Adds `Rows::into_vec` function.

# Verion 1.3.0

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
