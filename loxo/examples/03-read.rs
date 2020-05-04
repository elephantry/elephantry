mod serie {
    #[derive(Clone, Debug, loxo::Entity)]
    pub struct Entity {
        pub n: i32,
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

    find_by_pk(&loxo)?;
    find_all(&loxo)?;
    find_where(&loxo)?;
    count_where(&loxo)?;
    exist_where(&loxo)?;

    Ok(())
}

fn find_by_pk(loxo: &loxo::Pool) -> loxo::Result<()> {
    println!("# Find by primary key\n");

    let serie = loxo.find_by_pk::<serie::Model>(&loxo::pk!(n => 1))?;
    println!("{:?}\n", serie);

    Ok(())
}

fn find_all(loxo: &loxo::Pool) -> loxo::Result<()> {
    println!("# Find all\n");
    let series = loxo.find_all::<serie::Model>(Some("order by generate_series desc"))?;

    for serie in series {
        println!("{}", serie.n);
    }
    println!();

    Ok(())
}

fn find_where(loxo: &loxo::Pool) -> loxo::Result<()> {
    println!("# Find where\n");

    let series = loxo.find_where::<serie::Model>("generate_series > $1", &[&5], None)?;

    for serie in series {
        println!("{}", serie.n);
    }
    println!();

    Ok(())
}

fn count_where(loxo: &loxo::Pool) -> loxo::Result<()> {
    println!("# Count where\n");

    let n = loxo.count_where::<serie::Model>("generate_series % 2 = 0", &[])?;
    println!("{}\n", n);

    Ok(())
}

fn exist_where(loxo: &loxo::Pool) -> loxo::Result<()> {
    println!("# Exist where\n");

    let exist = loxo.exist_where::<serie::Model>("generate_series < 0", &[])?;
    println!("{}\n", exist);

    Ok(())
}
