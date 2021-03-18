#[derive(Debug, elephantry::Composite)]
pub struct Department {
    department_id: i32,
    name: String,
    parent_id: Option<i32>,
}

mod employee {
    #[derive(Debug, elephantry::Entity)]
    pub struct Entity {
        pub employee_id: i32,
        pub first_name: String,
        pub last_name: String,
        pub birth_date: chrono::NaiveDate,
        pub is_manager: bool,
        pub day_salary: bigdecimal::BigDecimal,
        pub departments: Vec<super::Department>,
    }

    pub struct Model<'a> {
        connection: &'a elephantry::Connection,
    }

    impl<'a> Model<'a> {
        pub fn employee_with_department(&self, id: i32) -> elephantry::Result<Entity> {
            use elephantry::{Model, Structure};

            let query = r#"
with recursive
    depts (department_id, name, parent_id) as (
        select d.department_id, d.name, d.parent_id from department d join {employee} e using(department_id) where e.employee_id = $1
        union all
        select d.department_id, d.name, d.parent_id from depts parent join department d on parent.parent_id = d.department_id
    )
select {employee_projection}
    from {employee} e, depts
    where e.employee_id = $1
    group by e.employee_id
"#;

            let projection = Self::create_projection()
                .unset_field("department_id")
                .add_field("departments", "array_agg(depts)")
                .alias("e");

            let sql = query
                .replace("{employee_projection}", &projection.to_string())
                .replace(
                    "{employee}",
                    <Self as elephantry::Model>::Structure::relation(),
                );

            Ok(self.connection.query::<Entity>(&sql, &[&id])?.get(0))
        }
    }

    impl<'a> elephantry::Model<'a> for Model<'a> {
        type Entity = Entity;
        type Structure = Structure;

        fn new(connection: &'a elephantry::Connection) -> Self {
            Self { connection }
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

fn main() -> elephantry::Result {
    pretty_env_logger::init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;
    elephantry.execute(include_str!("structure.sql"))?;

    let employee_with_department = elephantry
        .model::<employee::Model>()
        .employee_with_department(1)?;
    dbg!(employee_with_department);

    Ok(())
}
