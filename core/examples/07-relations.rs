mod employee {
    #[derive(Debug, elephantry::Entity)]
    pub struct Entity {
        pub employee_id: i32,
        pub first_name: String,
        pub last_name: String,
        pub birth_date: chrono::NaiveDate,
        pub is_manager: bool,
        pub day_salary: bigdecimal::BigDecimal,
        pub departments: Vec<String>,
    }

    pub struct Model<'a> {
        connection: &'a elephantry::Connection,
    }

    impl<'a> Model<'a> {
        pub fn employee_with_department(
            &self,
            id: i32,
        ) -> elephantry::Result<Entity> {
            use elephantry::{Model, Structure};

            let query = r#"
with recursive
    depts (name, parent_id, department_id) as (
        select {department_projection} from {department} d join {employee} e using(department_id) where e.employee_id = $1
        union all
        select {department_projection} from depts parent join {department} d on parent.parent_id = d.department_id
    )
select {employee_projection}
    from {employee} e, depts
    where e.employee_id = $1
    group by e.employee_id
"#;

            let projection = Self::create_projection()
                .unset_field("department_id")
                .add_field("departments", "array_agg(depts.name)")
                .alias("e");

            let sql = query
                .replace("{employee_projection}", &projection.to_string())
                .replace(
                    "{employee}",
                    <Self as elephantry::Model>::Structure::relation(),
                )
                .replace(
                    "{department_projection}",
                    "d.name, d.parent_id, d.department_id",
                    /* @FIXME
                    &super::department::Model::create_projection()
                        .alias("d")
                        .to_string(),
                    */
                )
                .replace(
                    "{department}",
                    super::department::Structure::relation(),
                );

            Ok(self.connection.query::<Entity>(&sql, &[&id])?.get(0))
        }
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

mod department {
    #[derive(Debug, elephantry::Entity)]
    pub struct Entity {
        department_id: i32,
        name: String,
        parent_id: Option<i32>,
    }

    pub struct Model {}

    impl<'a> elephantry::Model<'a> for Model {
        type Entity = Entity;
        type Structure = Structure;

        fn new(_: &'a elephantry::Connection) -> Self {
            Self {}
        }
    }

    pub struct Structure;

    impl elephantry::Structure for Structure {
        fn relation() -> &'static str {
            "department"
        }

        fn primary_key() -> &'static [&'static str] {
            &["department_id"]
        }

        fn definition() -> &'static [&'static str] {
            &["department_id", "name", "parent_id"]
        }
    }
}

fn main() -> elephantry::Result<()> {
    pretty_env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;
    elephantry.execute(include_str!("structure.sql"))?;

    let employee_with_department = elephantry
        .model::<employee::Model>()
        .employee_with_department(1)?;
    dbg!(employee_with_department);

    Ok(())
}
