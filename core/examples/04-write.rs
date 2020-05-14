mod employee {
    #[derive(Clone, Debug, elephantry::Entity)]
    pub struct Entity {
        pub employee_id: i32,
        pub first_name: String,
        pub last_name: String,
        pub birth_date: chrono::NaiveDate,
        pub is_manager: bool,
        pub day_salary: bigdecimal::BigDecimal,
        pub department_id: i32,
    }

    pub struct Model<'a> {
        connection: &'a elephantry::Connection,
    }

    impl<'a> elephantry::Model<'a> for Model<'a> {
        type Entity = Entity;
        type Structure = Structure;

        fn new(connection: &'a elephantry::Connection) -> Self {
            Self {
                connection,
            }
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

        fn definition() -> &'static [&'static str] {
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

    todo!();

    Ok(())
}
