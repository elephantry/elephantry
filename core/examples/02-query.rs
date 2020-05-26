mod employee {
    #[derive(Clone, Debug, elephantry::Entity)]
    pub struct Entity {
        pub employee_id: i32,
        pub first_name: String,
        pub last_name: String,
        pub birth_date: chrono::Date<chrono::offset::Utc>,
        pub is_manager: bool,
        pub day_salary: bigdecimal::BigDecimal,
        pub department_id: i32,
    }
}

fn main() -> elephantry::Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;
    elephantry.execute(include_str!("structure.sql"))?;

    let employees =
        elephantry.query::<employee::Entity>("select * from employee", &[])?;

    for employee in employees {
        dbg!(employee);
    }

    let total_salary =
        elephantry.query_one::<bigdecimal::BigDecimal>("select sum(day_salary) from employee", &[])?;

    dbg!(total_salary);

    Ok(())
}
