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
        pub age: elephantry::Interval,
    }

    pub struct Model;

    impl<'a> elephantry::Model<'a> for Model {
        type Entity = Entity;
        type Structure = Structure;

        fn new(_connection: &'a elephantry::Connection) -> Self {
            Self {}
        }

        fn create_projection() -> elephantry::Projection {
            Self::default_projection().add_field("age", "age(%:birth_date:%)")
        }
    }

    pub struct Structure;

    impl elephantry::Structure for Structure {
        fn relation() -> &'static str {
            "employee"
        }

        fn primary_key() -> &'static [&'static str] {
            &["employee_id"]
        }

        fn columns() -> &'static [&'static str] {
            &[
                "employee_id",
                "first_name",
                "last_name",
                "birth_date",
                "is_manager",
                "day_salary",
                "department_id",
            ]
        }
    }
}

fn main() -> elephantry::Result<()> {
    pretty_env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;
    elephantry.execute(include_str!("structure.sql"))?;

    let employees =
        elephantry.find_all::<employee::Model>(Some("order by age desc"))?;

    for employee in employees {
        dbg!(employee);
    }

    Ok(())
}
