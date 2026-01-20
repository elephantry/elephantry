#![warn(warnings)]

mod error;
mod generate;
mod inspect;

use clap::Parser;
use error::Error;

type Result<T = ()> = std::result::Result<T, crate::Error>;

#[derive(Debug, Parser)]
enum Opt {
    #[command(
        name = "inspect:database",
        about = "Show schemas in the current database"
    )]
    InspectDatabase {},
    #[command(name = "inspect:schema", about = "Show relations in a given schema")]
    InspectSchema {
        #[arg(default_value = "public")]
        schema: String,
    },
    #[command(name = "inspect:relation", about = "Display a relation information")]
    InspectRelation {
        relation: String,
        #[arg(default_value = "public")]
        schema: String,
    },
    #[command(name = "inspect:enums", about = "List enums")]
    InspectEnums {
        #[arg(default_value = "public")]
        schema: String,
    },
    #[command(name = "inspect:extensions", about = "List extensions")]
    InspectExtensions {
        #[arg(default_value = "public")]
        schema: String,
    },
    #[command(name = "inspect:domains", about = "List domains")]
    InspectDomains {
        #[arg(default_value = "public")]
        schema: String,
    },
    #[command(name = "inspect:composites", about = "List composites type")]
    InspectComposites {
        #[arg(default_value = "public")]
        schema: String,
    },
    #[command(
        name = "generate:schema-all",
        about = "Generate structure, model and entity file for all relations in a schema."
    )]
    GenerateSchema {
        #[arg(long, short = 'd', default_value = "src")]
        prefix_dir: String,
        #[arg(default_value = "public")]
        schema: String,
    },
    #[command(
        name = "generate:relation-all",
        about = "Generate structure, model and entity file for a given relation"
    )]
    GenerateRelation {
        #[arg(long, short = 'd', default_value = "src")]
        prefix_dir: String,
        relation: String,
        #[arg(default_value = "public")]
        schema: String,
    },
    #[command(name = "generate:entity", about = "Generate an Entity class")]
    GenerateEntity {
        #[arg(long, short = 'd', default_value = "src")]
        prefix_dir: String,
        relation: String,
        #[arg(default_value = "public")]
        schema: String,
    },
    #[command(name = "generate:enums", about = "Generate enums")]
    GenerateEnums {
        #[arg(long, short = 'd', default_value = "src")]
        prefix_dir: String,
        #[arg(default_value = "public")]
        schema: String,
    },
    #[command(name = "generate:composites", about = "Generate composites")]
    GenerateComposites {
        #[arg(long, short = 'd', default_value = "src")]
        prefix_dir: String,
        #[arg(default_value = "public")]
        schema: String,
    },
}

fn main() -> Result {
    envir::init();

    let opt = Opt::parse();
    let dsn = envir::get("DATABASE_URL")?;
    let elephantry = elephantry::Pool::new(&dsn).expect("Unable to connect to postgresql");

    match opt {
        Opt::InspectDatabase {} => inspect::database(&elephantry),
        Opt::InspectSchema { schema } => inspect::schema(&elephantry, &schema),
        Opt::InspectRelation { schema, relation } => {
            inspect::relation(&elephantry, &schema, &relation)
        }
        Opt::InspectEnums { schema } => inspect::enums(&elephantry, &schema),
        Opt::InspectExtensions { schema } => inspect::extensions(&elephantry, &schema),
        Opt::InspectDomains { schema } => inspect::domains(&elephantry, &schema),
        Opt::InspectComposites { schema } => inspect::composites(&elephantry, &schema),
        Opt::GenerateSchema { prefix_dir, schema } => {
            generate::schema(&elephantry, &prefix_dir, &schema)
        }
        Opt::GenerateRelation {
            prefix_dir,
            schema,
            relation,
        } => generate::relation(&elephantry, &prefix_dir, &schema, &relation),
        Opt::GenerateEntity {
            prefix_dir,
            schema,
            relation,
        } => generate::entity(&elephantry, &prefix_dir, &schema, &relation),
        Opt::GenerateEnums { prefix_dir, schema } => {
            generate::enums(&elephantry, &prefix_dir, &schema)
        }
        Opt::GenerateComposites { prefix_dir, schema } => {
            generate::composites(&elephantry, &prefix_dir, &schema)
        }
    }
}
