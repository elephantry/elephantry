#![warn(warnings)]

mod error;
mod generate;
mod inspect;

use clap::Parser;
use error::Error;

type Result<T = ()> = std::result::Result<T, crate::Error>;

#[derive(Debug, Parser)]
enum Opt {
    #[clap(
        name = "inspect:database",
        about = "Show schemas in the current database"
    )]
    InspectDatabase {},
    #[clap(name = "inspect:schema", about = "Show relations in a given schema")]
    InspectSchema {
        #[clap(default_value = "public")]
        schema: String,
    },
    #[clap(name = "inspect:relation", about = "Display a relation information")]
    InspectRelation {
        relation: String,
        #[clap(default_value = "public")]
        schema: String,
    },
    #[clap(name = "inspect:enums", about = "List enums")]
    InspectEnums {
        #[clap(default_value = "public")]
        schema: String,
    },
    #[clap(name = "inspect:domains", about = "List domains")]
    InspectDomains {
        #[clap(default_value = "public")]
        schema: String,
    },
    #[clap(name = "inspect:composites", about = "List composites type")]
    InspectComposites {
        #[clap(default_value = "public")]
        schema: String,
    },
    #[clap(
        name = "generate:schema-all",
        about = "Generate structure, model and entity file for all relations in a schema."
    )]
    GenerateSchema {
        #[clap(long, short = 'd', default_value = "src")]
        prefix_dir: String,
        #[clap(default_value = "public")]
        schema: String,
    },
    #[clap(
        name = "generate:relation-all",
        about = "Generate structure, model and entity file for a given relation"
    )]
    GenerateRelation {
        #[clap(long, short = 'd', default_value = "src")]
        prefix_dir: String,
        relation: String,
        #[clap(default_value = "public")]
        schema: String,
    },
    #[clap(name = "generate:entity", about = "Generate an Entity class")]
    GenerateEntity {
        #[clap(long, short = 'd', default_value = "src")]
        prefix_dir: String,
        relation: String,
        #[clap(default_value = "public")]
        schema: String,
    },
    #[clap(name = "generate:enums", about = "Generate enums")]
    GenerateEnums {
        #[clap(long, short = 'd', default_value = "src")]
        prefix_dir: String,
        #[clap(default_value = "public")]
        schema: String,
    },
    #[clap(name = "generate:composites", about = "Generate composites")]
    GenerateComposites {
        #[clap(long, short = 'd', default_value = "src")]
        prefix_dir: String,
        #[clap(default_value = "public")]
        schema: String,
    },
}

fn main() -> Result {
    dotenv::dotenv().ok();

    let opt = Opt::parse();
    let dsn = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env variable");
    let elephantry = elephantry::Pool::new(&dsn).expect("Unable to connect to postgresql");

    match opt {
        Opt::InspectDatabase {} => inspect::database(&elephantry),
        Opt::InspectSchema { schema } => inspect::schema(&elephantry, &schema),
        Opt::InspectRelation { schema, relation } => {
            inspect::relation(&elephantry, &schema, &relation)
        }
        Opt::InspectEnums { schema } => inspect::enums(&elephantry, &schema),
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
