# Elephantry

[![Crates.io](https://img.shields.io/crates/v/elephantry)](https://crates.io/crates/elephantry)
[![docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/elephantry)
[![Github actions](https://github.com/elephantry/elephantry/workflows/.github/workflows/ci.yml/badge.svg)](https://github.com/elephantry/elephantry/actions?query=workflow%3A.github%2Fworkflows%2Fci.yml)
[![Build Status](https://gitlab.com/elephantry/elephantry/badges/master/pipeline.svg)](https://gitlab.com/elephantry/elephantry/commits/master)

When Rust meets PostgreSQL.

## Getting Started

See [quickstart](docs/quickstart.md) and [examples](core/examples).

## Quick overview

Elephantry is an OMM (object model manager) dedicated to PostgreSQL design to
handle from simple to complex queries.

```rust
// Connect
let elephantry = elephantry::Pool::new("postgres://localhost")?;

// Simple query
let rows = elephantry.execute("select n from generate_series(1, 10)")?;

for row in &rows {
    let n: i32 = row.get("n");
    println!("{}", n);
}

// Read entities
let entity = elephantry.find_by_pk::<Model>(elephantry::pk!(id))?;
let entities = elephantry.find_all::<Model>(None)?;
let entities = elephantry.find_where::<Model>("deleted = $1", &[&false], None)?;

// Write entities
elephantry.insert_one(entity)?;
elephantry.update_one(elephantry::pk!{id => entity.id}, entity)?;
elephantry.delete_one(entity)?;
elephantry.delete_where("deleted = $1", &[&true])?;
```

## Features

- config-support — adds support for [config](https://crates.io/crates/config)
    layered configuration system;
- r2d2 — adds support for [r2d2](https://crates.io/crates/r2d2) generic
    connection pool;
- rocket — adds support for
    [rocket](https://rocket.rs/v0.4/guide/state/#databases) web framewok;
- serde-support — adds support for de/serialization via
    [serde](https://serde.rs/).

### Types

- bit — adds support for
    [bit](https://www.postgresql.org/docs/current/datatype-bit.html) type;
- date — adds support for
    [date](https://www.postgresql.org/docs/current/datatype-datetime.html) type;
- geo — adds support for
    [geometric](https://www.postgresql.org/docs/current/datatype-geometric.html)
    type;
- json — adds support for
    [json](https://www.postgresql.org/docs/current/datatype-json.html) type;
- money — adds support for
    [money](https://www.postgresql.org/docs/current/datatype-money.html) type;
- net — adds support for
    [network](https://www.postgresql.org/docs/current/datatype-net-types.html)
    type;
- numeric — adds support for
    [numeric](https://www.postgresql.org/docs/current/datatype-numeric.html)
    type;
- time — adds support for
    [time](https://www.postgresql.org/docs/current/datatype-datetime.html) type;
- uuid — adds support for
    [uuid](https://www.postgresql.org/docs/current/datatype-uuid.html) type;
- xml — adds support for
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
    application builds with actix to display statitics.

If you want to add your project here, please [create a pull
request](https://github.com/elephantry/elephantry/pulls).
