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

    let elephantry = elephantry::Pool::new("postgres://localhost")?;
    elephantry.execute(include_str!("database.sql"))?;

    Ok(())
}
