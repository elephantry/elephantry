fn main() -> elephantry::Result {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;
    elephantry.execute(include_str!("structure.sql"))?;

    let results = elephantry.execute("select * from department")?;

    for result in &results {
        let name: String = result.get("name");
        println!("- {name}");
    }

    let parent_id: Option<i32> = results.get(0).get("parent_id");
    dbg!(parent_id);

    match results.get(0).try_get::<i32>("parent_id") {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {err:?}"),
    }

    match results.get(0).try_get::<i32>("missing_field") {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {err:?}"),
    }

    Ok(())
}
