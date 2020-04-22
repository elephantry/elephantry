mod serie {
    #[derive(Clone, Debug, romm::Entity)]
    pub struct Entity {
        n: i32,
    }

    pub struct Model;

    impl<'a> romm::Model<'a> for Model {
        type Entity = Entity;
        type RowStructure = Structure;

        fn new(_: &'a romm::Connection) -> Self {
            Self {}
        }
    }

    pub struct Structure;

    impl romm::row::Structure for Structure {
        fn relation() -> &'static str {
            "generate_series(1, 10)"
        }

        fn primary_key() -> &'static [&'static str] {
            &["n"]
        }

        fn definition() -> std::collections::HashMap<&'static str, romm::Row> {
            maplit::hashmap! {
                "n" => romm::Row {
                    content: "%:generate_series:%",
                    ty: romm::pq::ty::INT4,
                },
            }
        }
    }
}

fn main() -> romm::Result<()> {
    let romm = romm::Romm::new().add_default("romm", "postgres://sanpi@localhost/romm")?;
    let connection = romm.default().unwrap();

    let series = connection.find_all::<serie::Model>()?;

    for serie in series {
        dbg!(serie);
    }

    Ok(())
}
