#[derive(Debug, elephantry::Enum)]
enum Department {
    // By default the variant value is transpose as string without transformation:
    Direction,
    // If you want change this, you can use the `value` attribute:
    #[elephantry(value = "Siège")]
    Siege,
}

fn main() -> elephantry::Result {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;
    elephantry.execute(include_str!("structure.sql"))?;

    let departments = elephantry.query::<Department>(
        "select distinct name from department where name = $*",
        &[&Department::Siege],
    )?;

    for department in departments {
        dbg!(department);
    }

    Ok(())
}
