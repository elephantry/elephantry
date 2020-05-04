mod serie {
    #[derive(Clone, Debug, elephantry::Entity)]
    pub struct Entity {
        pub n: i32,
        pub even: bool,
    }

    pub struct Model;

    impl<'a> elephantry::Model<'a> for Model {
        type Entity = Entity;
        type Structure = Structure;

        fn new(_: &'a elephantry::Connection) -> Self {
            Self {}
        }

        fn create_projection() -> elephantry::Projection {
            Self::default_projection().add_field("even", "%:n:% % 2 = 0")
        }
    }

    pub struct Structure;

    impl elephantry::Structure for Structure {
        fn relation() -> &'static str {
            "serie"
        }

        fn primary_key() -> &'static [&'static str] {
            &["n"]
        }

        fn definition() -> &'static [&'static str] {
            &["n"]
        }
    }
}

fn main() -> elephantry::Result<()> {
    pretty_env_logger::init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;
    elephantry.execute(include_str!("database.sql"))?;

    let series = elephantry.find_all::<serie::Model>(None)?;

    for serie in series {
        dbg!(serie);
    }

    Ok(())
}
