mod serie {
    #[derive(Clone, Debug, elephantry::Entity)]
    pub struct Entity {
        pub generate_series: i32,
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
            "generate_series(1, 10)"
        }

        fn primary_key() -> &'static [&'static str] {
            &["generate_series"]
        }

        fn definition() -> &'static [&'static str] {
            &["generate_series"]
        }
    }
}

fn main() -> elephantry::Result<()> {
    pretty_env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;
    elephantry.execute(include_str!("database.sql"))?;

    Ok(())
}
