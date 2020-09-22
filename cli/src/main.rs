#![warn(rust_2018_idioms)]

mod error;
mod generate;
mod inspect;
mod pq;

use error::Error;
use structopt::StructOpt;

type Result<T> = std::result::Result<T, crate::Error>;

#[derive(Debug, StructOpt)]
enum Opt {
    #[structopt(
        name = "inspect:database",
        about = "Show schemas in the current database"
    )]
    InspectDatabase {},
    #[structopt(
        name = "inspect:schema",
        about = "Show relations in a given schema"
    )]
    InspectSchema {
        #[structopt(default_value = "public")]
        schema: String,
    },
    #[structopt(
        name = "inspect:relation",
        about = "Display a relation information"
    )]
    InspectRelation {
        relation: String,
        #[structopt(default_value = "public")]
        schema: String,
    },
    #[structopt(name = "inspect:enums", about = "List enums")]
    InspectEnums {
        #[structopt(default_value = "public")]
        schema: String,
    },
    #[structopt(name = "inspect:domains", about = "List domains")]
    InspectDomains {
        #[structopt(default_value = "public")]
        schema: String,
    },
    #[structopt(
        name = "generate:schema-all",
        about = "Generate structure, model and entity file for all relations in a schema."
    )]
    GenerateSchema {
        #[structopt(long, short = "d", default_value = "src")]
        prefix_dir: String,
        #[structopt(default_value = "public")]
        schema: String,
    },
    #[structopt(
        name = "generate:relation-all",
        about = "Generate structure, model and entity file for a given relation"
    )]
    GenerateRelation {
        #[structopt(long, short = "d", default_value = "src")]
        prefix_dir: String,
        relation: String,
        #[structopt(default_value = "public")]
        schema: String,
    },
    #[structopt(name = "generate:entity", about = "Generate an Entity class")]
    GenerateEntity {
        #[structopt(long, short = "d", default_value = "src")]
        prefix_dir: String,
        relation: String,
        #[structopt(default_value = "public")]
        schema: String,
    },
    #[structopt(name = "generate:enums", about = "Generate enums")]
    GenerateEnums {
        #[structopt(long, short = "d", default_value = "src")]
        prefix_dir: String,
        #[structopt(default_value = "public")]
        schema: String,
    },
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let opt = Opt::from_args();
    let dsn = std::env::var("DATABASE_URL")
        .expect("Missing DATABASE_URL env variable");
    let elephantry =
        elephantry::Pool::new(&dsn).expect("Unable to connect to postgresql");
    let connection = elephantry.get_default().unwrap();

    match opt {
        Opt::InspectDatabase {} => inspect::database(&connection),
        Opt::InspectSchema {
            schema,
        } => inspect::schema(&connection, &schema),
        Opt::InspectRelation {
            schema,
            relation,
        } => inspect::relation(&connection, &schema, &relation),
        Opt::InspectEnums {
            schema,
        } => inspect::enums(&connection, &schema),
        Opt::InspectDomains {
            schema,
        } => inspect::domains(&connection, &schema),
        Opt::GenerateSchema {
            prefix_dir,
            schema,
        } => generate::schema(&connection, &prefix_dir, &schema)?,
        Opt::GenerateRelation {
            prefix_dir,
            schema,
            relation,
        } => generate::relation(&connection, &prefix_dir, &schema, &relation)?,
        Opt::GenerateEntity {
            prefix_dir,
            schema,
            relation,
        } => generate::entity(&connection, &prefix_dir, &schema, &relation)?,
        Opt::GenerateEnums {
            prefix_dir,
            schema,
        } => generate::enums(&connection, &prefix_dir, &schema)?,
    }

    Ok(())
}
