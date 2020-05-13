# Elephantry

[![Crates.io](https://img.shields.io/crates/v/elephantry)](https://crates.io/crates/elephantry)
[![docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/elephantry)
[![Build Status](https://gitlab.com/sanpi/elephantry/badges/master/pipeline.svg)](https://gitlab.com/sanpi/elephantry/commits/master)

![](docs/logo.png)

When Rust meets PostgreSQL.

## Getting Started

See [examples](core/examples).

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
