fn main() -> romm::Result<()> {
    let romm = romm::Romm::new().add_default("romm", "postgres://sanpi@localhost/romm")?;
    let connection = romm.default().unwrap();

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
