mod serie {
    #[derive(Clone, Debug, loxo::Entity)]
    pub struct Entity {
        n: i32,
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

    impl loxo::row::Structure for Structure {
        fn relation() -> &'static str {
            "generate_series(1, 10)"
        }

        fn primary_key() -> &'static [&'static str] {
            &["n"]
        }

        fn definition() -> std::collections::HashMap<&'static str, &'static str> {
            maplit::hashmap! {
                "n" => "%:generate_series:%",
            }
        }
    }
}

fn main() -> loxo::Result<()> {
    let loxo = loxo::Loxo::new().add_default("loxo", "postgres://sanpi@localhost/loxo")?;
    let connection = loxo.default().unwrap();

    let series = connection.find_all::<serie::Model>()?;

    for serie in series {
        dbg!(serie);
    }

    Ok(())
}
