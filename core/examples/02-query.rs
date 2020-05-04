#[derive(Clone, Debug, elephantry::Entity)]
struct Serie {
    n: i32,
}

fn main() -> elephantry::Result<()> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;

    let series = elephantry.query::<Serie>(
        "select generate_series as n from generate_series(1, 10)",
        &[],
    )?;

    for serie in series {
        dbg!(serie);
    }

    Ok(())
}
