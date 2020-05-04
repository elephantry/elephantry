#[derive(Clone, Debug, elephantry::Entity)]
struct Serie {
    n: i32,
}

fn main() -> elephantry::Result<()> {
    let elephantry = elephantry::Pool::new("postgres://localhost")?;

    let series = elephantry.query::<Serie>(
        "select generate_series as n from generate_series(1, 10)",
        &[],
    )?;

    for serie in series {
        dbg!(serie);
    }

    Ok(())
}
