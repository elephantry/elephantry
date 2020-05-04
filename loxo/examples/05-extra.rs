mod serie {
    #[derive(Clone, Debug, loxo::Entity)]
    pub struct Entity {
        pub n: i32,
        pub even: bool,
    }

    pub struct Model;

    impl<'a> loxo::Model<'a> for Model {
        type Entity = Entity;
        type Structure = Structure;

        fn new(_: &'a loxo::Connection) -> Self {
            Self {}
        }

        fn create_projection() -> loxo::Projection {
            Self::default_projection()
                .add_field("even", "%:n:% % 2 = 0")
        }
    }

    pub struct Structure;

    impl loxo::Structure for Structure {
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

fn main() -> loxo::Result<()> {
    pretty_env_logger::init();

    let loxo = loxo::Pool::new("postgres://localhost")?;
    loxo.execute(include_str!("database.sql"))?;

    let series = loxo.find_all::<serie::Model>(None)?;

    for serie in series {
        dbg!(serie);
    }

    Ok(())
}
