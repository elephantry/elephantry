mod employee {
    #[derive(Clone, Debug, elephantry::Entity)]
    pub struct Entity {
        pub employee_id: Option<i32>,
        pub first_name: String,
        pub last_name: String,
        pub birth_date: chrono::NaiveDate,
        pub is_manager: bool,
        pub day_salary: bigdecimal::BigDecimal,
        pub department_id: i32,
    }

    pub struct Model;

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

    let entity = insert(&elephantry)?;
    update(&elephantry, &entity)?;
    delete(&elephantry, &entity)?;

    Ok(())
}

fn insert(elephantry: &elephantry::Pool) -> elephantry::Result<employee::Entity> {
    let employee = employee::Entity {
        employee_id: None,
        first_name: "First name".to_string(),
        last_name: "Last name".to_string(),
        birth_date: chrono::NaiveDate::from_ymd(1952, 03, 21),
        is_manager: false,
        day_salary: 10_000.into(),
        department_id: 3,
    };

    let inserted_entity = elephantry.insert_one::<employee::Model>(&employee)?;
    dbg!(&inserted_entity);

    let upsert_nothing =
        elephantry.upsert_one::<employee::Model>(&inserted_entity, "(employee_id)", "nothing")?;
    dbg!(&upsert_nothing);

    let upsert_update = elephantry.upsert_one::<employee::Model>(
        &inserted_entity,
        "(employee_id)",
        "update set employee_id = default",
    )?;
    dbg!(&upsert_update);

    Ok(inserted_entity)
}

fn update(elephantry: &elephantry::Pool, entity: &employee::Entity) -> elephantry::Result {
    let mut entity = entity.clone();
    entity.day_salary = 20_000.into();

    let updated_entity = elephantry.update_one::<employee::Model>(
        &elephantry::pk!(employee_id => entity.employee_id),
        &entity,
    )?;
    dbg!(updated_entity);

    let mut data = std::collections::HashMap::new();
    data.insert("is_manager".to_string(), &true as &dyn elephantry::ToSql);

    let updated_entity = elephantry.update_by_pk::<employee::Model>(
        &elephantry::pk!(employee_id => entity.employee_id),
        &data,
    )?;
    dbg!(updated_entity);

    Ok(())
}

fn delete(elephantry: &elephantry::Pool, entity: &employee::Entity) -> elephantry::Result {
    let deleted_entity = elephantry.delete_one::<employee::Model>(&entity)?;
    dbg!(deleted_entity);

    let deleted_entity = elephantry
        .delete_by_pk::<employee::Model>(&elephantry::pk!(employee_id => entity.employee_id))?;
    dbg!(deleted_entity);

    elephantry.delete_where::<employee::Model>("employee_id = $1", &[&entity.employee_id])?;

    Ok(())
}
