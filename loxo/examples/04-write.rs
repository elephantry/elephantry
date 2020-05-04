mod serie {
    #[derive(Clone, Debug, loxo::Entity)]
    pub struct Entity {
        pub generate_series: i32,
    }

    pub struct Model;

    impl<'a> loxo::Model<'a> for Model {
        type Entity = Entity;
        type Structure = Structure;

        fn new(_: &'a loxo::Connection) -> Self {
            Self {}
        }
    }

    pub struct Structure;

    impl loxo::Structure for Structure {
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

fn main() -> loxo::Result<()> {
    pretty_env_logger::init();

    let loxo = loxo::Pool::new("postgres://localhost")?;
    loxo.execute(include_str!("database.sql"))?;

    Ok(())
}
