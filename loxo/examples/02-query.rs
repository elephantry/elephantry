#[derive(Clone, Debug, loxo::Entity)]
struct Serie {
    n: i32,
}

fn main() -> loxo::Result<()> {
    let loxo = loxo::Pool::new("postgres://localhost")?;

    let series = loxo.query::<Serie>(
        "select generate_series as n from generate_series(1, 10)",
        &[],
    )?;

    for serie in series {
        dbg!(serie);
    }

    Ok(())
}
