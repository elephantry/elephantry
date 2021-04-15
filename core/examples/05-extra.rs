mod employee {
    #[derive(Debug, elephantry::Entity)]
    #[elephantry(model = "Model", structure = "Structure", relation = "employee")]
    pub struct Entity {
        #[elephantry(pk)]
        pub employee_id: i32,
        pub first_name: String,
        pub last_name: String,
        pub birth_date: chrono::NaiveDate,
        pub is_manager: bool,
        pub day_salary: bigdecimal::BigDecimal,
        pub department_id: i32,
        #[elephantry(virtual = "age(%:birth_date:%)")]
        pub age: elephantry::Interval,
    }
}

fn main() -> elephantry::Result {
    pretty_env_logger::init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;
    elephantry.execute(include_str!("structure.sql"))?;

    let employees = elephantry.find_all::<employee::Model>(Some("order by age desc"))?;

    for employee in employees {
        dbg!(employee);
    }

    Ok(())
}
