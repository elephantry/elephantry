mod employee {
    #[derive(Debug, elephantry::Entity)]
    pub struct Entity {
        pub employee_id: i32,
        pub first_name: String,
        pub last_name: String,
        pub birth_date: chrono::NaiveDate,
        pub is_manager: bool,
        pub day_salary: bigdecimal::BigDecimal,
        pub department_id: i32,
    }
}

#[async_std::main]
async fn main() -> elephantry::Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;

    elephantry
        .r#async()
        .execute(include_str!("structure.sql"))
        .await?;

    let employees = elephantry
        .r#async()
        .query::<employee::Entity>("select * from employee", &[])
        .await?;

    for employee in employees {
        dbg!(employee);
    }

    let total_salary = elephantry
        .r#async()
        .query_one::<bigdecimal::BigDecimal>(
            "select sum(day_salary) from employee",
            &[],
        )
        .await?;

    dbg!(total_salary);

    Ok(())
}
