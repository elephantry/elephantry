mod employee {
    #[derive(Clone, Debug, elephantry::Entity)]
    #[elephantry(model = "Model", structure = "Structure", relation = "employee")]
    pub struct Entity {
        #[elephantry(column = "employee_id", pk)]
        pub id: i32,
        pub first_name: String,
        pub last_name: String,
        pub birth_date: chrono::NaiveDate,
        pub is_manager: bool,
        pub day_salary: bigdecimal::BigDecimal,
        pub department_id: i32,
    }

    impl Entity {
        pub fn new(id: i32) -> Self {
            Self {
                id,
                first_name: format!("first name {id}"),
                last_name: format!("last name {id}"),
                birth_date: chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
                is_manager: false,
                day_salary: 0.into(),
                department_id: 1,
            }
        }
    }
}

fn main() -> elephantry::Result {
    env_logger::init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;
    elephantry.execute(include_str!("structure.sql"))?;

    let employees = (15..10_000).into_iter().map(employee::Entity::new);

    elephantry.copy::<employee::Model, _>(employees)?;

    let count = elephantry.count_where::<employee::Model>("true = true", &[])?;
    dbg!(count);

    Ok(())
}
