#![allow(unused_must_use)]

fn main() -> loxo::Result<()> {
    let loxo = loxo::Loxo::new("postgres://localhost")?;

    let results = loxo.execute(
        "select generate_series as n, null as null_field from generate_series(1, 10)",
    )?;

    for result in &results {
        let n: i32 = result.get("n");
        dbg!(n);
    }

    let null_field: Option<i32> = results.get(0).get("null_field");
    dbg!(null_field);

    match results.get(0).try_get::<i32>("null_field") {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {:?}", err),
    }

    match results.get(0).try_get::<i32>("missing_field") {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {:?}", err),
    }

    Ok(())
}
