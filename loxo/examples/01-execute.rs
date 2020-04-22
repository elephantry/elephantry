fn main() -> loxo::Result<()> {
    let loxo = loxo::Loxo::new().add_default("loxo", "postgres://sanpi@localhost/loxo")?;
    let connection = loxo.default().unwrap();

    let results = connection.execute(
        "select generate_series as n from generate_series(1, 10)",
        &[],
    )?;

    for result in results {
        let n: i32 = result.get("n");
        dbg!(n);

        let missing_field: Option<i32> = result.get("missing_field");
        dbg!(missing_field);
    }

    Ok(())
}
