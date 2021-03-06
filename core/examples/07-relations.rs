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
        #[elephantry(virtual)]
        pub departments: Vec<String>,
    }

    impl<'a> Model<'a> {
        pub fn employee_with_department(&self, id: i32) -> elephantry::Result<Entity> {
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
                .replace("{department}", super::department::Structure::relation());

            Ok(self.connection.query::<Entity>(&sql, &[&id])?.get(0))
        }
    }
}

mod department {
    #[derive(Debug, elephantry::Entity)]
    #[elephantry(model = "Model", structure = "Structure", relation = "department")]
    pub struct Entity {
        #[elephantry(pk)]
        department_id: i32,
        name: String,
        parent_id: Option<i32>,
    }
}

fn main() -> elephantry::Result {
    env_logger::init();

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
