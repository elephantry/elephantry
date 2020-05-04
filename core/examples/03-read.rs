mod serie {
    #[derive(Clone, Debug, elephantry::Entity)]
    pub struct Entity {
        pub n: i32,
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

    find_by_pk(&elephantry)?;
    find_all(&elephantry)?;
    find_where(&elephantry)?;
    count_where(&elephantry)?;
    exist_where(&elephantry)?;

    Ok(())
}

fn find_by_pk(elephantry: &elephantry::Pool) -> elephantry::Result<()> {
    println!("# Find by primary key\n");

    let serie = elephantry.find_by_pk::<serie::Model>(&elephantry::pk!(n => 1))?;
    println!("{:?}\n", serie);

    Ok(())
}

fn find_all(elephantry: &elephantry::Pool) -> elephantry::Result<()> {
    println!("# Find all\n");
    let series = elephantry.find_all::<serie::Model>(Some("order by generate_series desc"))?;

    for serie in series {
        println!("{}", serie.n);
    }
    println!();

    Ok(())
}

fn find_where(elephantry: &elephantry::Pool) -> elephantry::Result<()> {
    println!("# Find where\n");

    let series = elephantry.find_where::<serie::Model>("generate_series > $1", &[&5], None)?;

    for serie in series {
        println!("{}", serie.n);
    }
    println!();

    Ok(())
}

fn count_where(elephantry: &elephantry::Pool) -> elephantry::Result<()> {
    println!("# Count where\n");

    let n = elephantry.count_where::<serie::Model>("generate_series % 2 = 0", &[])?;
    println!("{}\n", n);

    Ok(())
}

fn exist_where(elephantry: &elephantry::Pool) -> elephantry::Result<()> {
    println!("# Exist where\n");

    let exist = elephantry.exist_where::<serie::Model>("generate_series < 0", &[])?;
    println!("{}\n", exist);

    Ok(())
}
