# Elephantry

[![Crates.io](https://img.shields.io/crates/v/elephantry)](https://crates.io/crates/elephantry)
[![docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/elephantry)
[![Github actions](https://github.com/elephantry/elephantry/workflows/.github/workflows/ci.yml/badge.svg)](https://github.com/elephantry/elephantry/actions?query=workflow%3A.github%2Fworkflows%2Fci.yml)
[![pipeline status](https://gitlab.com/elephantry/elephantry/badges/main/pipeline.svg)](https://gitlab.com/elephantry/elephantry/-/commits/main)

When Rust meets PostgreSQL.

## Getting Started

See [quickstart](https://elephantry.github.io/documentation/quickstart/) and [examples](core/examples).

## Quick overview

Elephantry is an OMM (object model manager) dedicated to PostgreSQL design to
handle from simple to complex queries.

```rust
let database_url = std::env::var("DATABASE_URL")
    .unwrap_or_else(|_| "postgres://localhost".to_string());

// Connect
let elephantry = elephantry::Pool::new(&database_url)?;

// Simple query
let rows = elephantry.execute("select id from entity")?;

for row in &rows {
    let id: i32 = row.get("id");
    println!("{}", id);
}

// Define entity
#[derive(elephantry::Entity)]
#[elephantry(model = "Model", structure = "Structure")]
struct Entity {
    #[elephantry(pk)]
    id: u16,
    deleted: bool,
}

// Read entities
let entity = elephantry.find_by_pk::<Model>(&elephantry::pk!(id))?;
let entities = elephantry.find_all::<Model>(None)?;
let entities = elephantry.find_where::<Model>("deleted = $1", &[&false], None)?;

// Write entities
elephantry.insert_one::<Model>(&entity)?;
elephantry.update_one::<Model>(&elephantry::pk!{id => entity.id}, &entity)?;
elephantry.delete_one::<Model>(&entity)?;
elephantry.delete_where::<Model>("deleted = $1", &[&true])?;
```

## Features

- `all-types` — enables all type features (see below);
- `arbitrary` — add support for [arbitrary
    crate](https://crates.io/crates/arbitrary);
- `config` — adds support for [config](https://crates.io/crates/config)
    layered configuration system;
- `r2d2` — adds support for [r2d2](https://crates.io/crates/r2d2) generic
    connection pool;
- `rocket` — adds support for
    [rocket](https://rocket.rs/v0.4/guide/state/#databases) web framewok;
- `serde` — adds support for de/serialization via [serde](https://serde.rs/).

### Types

- `bit` — adds support for
    [bit](https://www.postgresql.org/docs/current/datatype-bit.html) type;
- `date` — adds support for
    [date](https://www.postgresql.org/docs/current/datatype-datetime.html) type;
- `geo` — adds support for
    [geometric](https://www.postgresql.org/docs/current/datatype-geometric.html)
    type;
- `json` — adds support for
    [json](https://www.postgresql.org/docs/current/datatype-json.html) type;
- `multirange` — adds support for
    [multirange](https://www.postgresql.org/docs/14/rangetypes.html) type
    (postgresql >= 14);
- `money` — adds support for
    [money](https://www.postgresql.org/docs/current/datatype-money.html) type;
- `net` — adds support for
    [network](https://www.postgresql.org/docs/current/datatype-net-types.html)
    type;
- `numeric` — adds support for
    [numeric](https://www.postgresql.org/docs/current/datatype-numeric.html)
    type;
- `time` — adds support for
    [time](https://www.postgresql.org/docs/current/datatype-datetime.html) type;
- `uuid` — adds support for
    [uuid](https://www.postgresql.org/docs/current/datatype-uuid.html) type;
- `xml` — adds support for
    [xml](https://www.postgresql.org/docs/current/datatype-xml.html) type.


## Projects using Elephantry

- [todo](https://github.com/elephantry/todo) — Todo rocket example app;
- [explain](https://github.com/sanpii/explain) — A CLI tool transforms
    postgresql explain to a graph;
- [sav](https://github.com/sanpii/sav) — A simple CRUD application to archive
    bought item waranty build with rocket;
- [out of gafam](https://github.com/sanpii/out-of-gafam) — Generate RSS feed for
    GAFAM (youtube, facebook, instagram and twitter) using actix.
- [captainstat](https://github.com/sanpii/captainstat) — Another simple
    application builds with actix to display statitics;
- [oxfeed](https://github.com/sanpii/oxfeed) — A feed reader with an actix API
    and yew front.

If you want to add your project here, please [create a pull
request](https://github.com/elephantry/elephantry/pulls).
