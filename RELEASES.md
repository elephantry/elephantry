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
