mod employee {
    #[derive(Debug, elephantry::Entity)]
    #[elephantry(model = "Model", structure = "Structure", relation = "employee")]
    pub struct Entity {
        #[elephantry(pk)]
        pub employee_id: Option<i32>,
        pub first_name: String,
        pub last_name: String,
        pub birth_date: chrono::NaiveDate,
        pub is_manager: bool,
        pub day_salary: bigdecimal::BigDecimal,
        pub department_id: i32,
    }

    impl<'a> Model<'a> {
        pub fn new_employee(
            &self,
            employee: &Entity,
            department: &str,
        ) -> elephantry::Result<Entity> {
            let transaction = self.connection.transaction();

            transaction.start()?;
            transaction.set_deferrable(
                Some(vec!["employee_department_id_fkey"]),
                elephantry::transaction::Constraints::Deferred,
            )?;

            let mut employee = self.connection.insert_one::<Self>(employee)?;
            let department = self
                .connection
                .find_where::<super::department::Model>("name = $*", &[&department], None)?
                .nth(0)
                .unwrap();
            employee.department_id = department.department_id;

            let employee = self
                .connection
                .update_one::<Self>(
                    &elephantry::pk! { employee_id => employee.employee_id },
                    &employee,
                )?
                .unwrap();

            transaction.commit()?;

            Ok(employee)
        }
    }
}

mod department {
    #[derive(Debug, elephantry::Entity)]
    #[elephantry(model = "Model", structure = "Structure", relation = "department")]
    pub struct Entity {
        #[elephantry(pk)]
        pub department_id: i32,
        pub name: String,
        pub parent_id: Option<i32>,
    }
}

fn main() -> elephantry::Result {
    pretty_env_logger::init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;
    elephantry.execute(include_str!("structure.sql"))?;

    let employee = elephantry.model::<employee::Model>().new_employee(
        &employee::Entity {
            employee_id: None,
            first_name: "First name".to_string(),
            last_name: "Last name".to_string(),
            birth_date: chrono::NaiveDate::from_ymd(1990, 1, 1),
            is_manager: true,
            day_salary: 1_000.into(),
            department_id: -1,
        },
        "Direction",
    )?;

    dbg!(employee);

    Ok(())
}
