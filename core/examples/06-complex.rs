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
    }

    impl<'a> Model<'a> {
        pub fn managers_salary(&self) -> elephantry::Result<f32> {
            let query = "select sum(day_salary) from employee where is_manager";

            let result = self.connection.execute(query)?.get(0).get("sum");

            Ok(result)
        }
    }
}

fn main() -> elephantry::Result {
    env_logger::init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;
    elephantry.execute(include_str!("structure.sql"))?;

    let model = elephantry.model::<employee::Model>();
    let managers_salary = model.managers_salary()?;

    dbg!(managers_salary);

    Ok(())
}
