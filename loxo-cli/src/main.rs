use structopt::StructOpt;

mod generate;
mod inspect;

#[derive(Debug, StructOpt)]
enum Opt {
    #[structopt(
        name = "inspect:database",
        about = "Show schemas in the current database"
    )]
    InspectDatabase {},
    #[structopt(name = "inspect:schema", about = "Show relations in a given schema")]
    InspectSchema {
        #[structopt(default_value = "public")]
        schema: String,
    },
    #[structopt(name = "inspect:relation", about = "Display a relation information")]
    InspectRelation {
        relation: String,
        #[structopt(default_value = "public")]
        schema: String,
    },
    #[structopt(name = "generate:schema-all", about = "Generate structure, model and entity file for all relations in a schema.")]
    GenerateSchema {
        #[structopt(default_value = "public")]
        schema: String,
    },
    #[structopt(name = "generate:relation-all", about = "Generate structure, model and entity file for a given relation")]
    GenerateRelation {
        relation: String,
        #[structopt(default_value = "public")]
        schema: String,
    },
    #[structopt(name = "generate:entity", about = "Generate an Entity class")]
    GenerateEntity {
        relation: String,
        #[structopt(default_value = "public")]
        schema: String,
    },
}

fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let opt = Opt::from_args();
    let dsn = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env variable");
    let loxo = loxo::Loxo::new()
        .add_default("db", &dsn)
        .expect("Unable to connect to postgresql");
    let connection = loxo.default().unwrap();

    match opt {
        Opt::InspectDatabase {} => inspect::database(&connection),
        Opt::InspectSchema { schema } => inspect::schema(&connection, &schema),
        Opt::InspectRelation { schema, relation } => {
            inspect::relation(&connection, &schema, &relation)
        }
        Opt::GenerateSchema { schema } => {
            generate::schema(&connection, &schema)?
        }
        Opt::GenerateRelation { schema, relation } => {
            generate::relation(&connection, &schema, &relation)?
        }
        Opt::GenerateEntity { schema, relation } => {
            generate::entity(&connection, &schema, &relation)?
        }
    }

    Ok(())
}
