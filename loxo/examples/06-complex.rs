mod serie {
    pub struct Model<'a> {
        connection: &'a loxo::Connection,
    }

    impl<'a> loxo::Model<'a> for Model<'a> {
        type Entity = std::collections::HashMap<String, i32>;
        type Structure = Structure;

        fn new(connection: &'a loxo::Connection) -> Self {
            Self { connection }
        }
    }

    impl<'a> Model<'a> {
        pub fn even_sum(&self) -> loxo::Result<i32> {
            let query = "select sum(n) from serie where n % 2 = 0";

            let result = self.connection.execute(query)?
                .get(0)
                .get("sum");

            Ok(result)
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

    let loxo = loxo::Loxo::new("postgres://localhost")?;
    loxo.execute(include_str!("database.sql"))?;

    let model = loxo.model::<serie::Model>();
    let sum = model.even_sum()?;

    dbg!(sum);

    Ok(())
}
