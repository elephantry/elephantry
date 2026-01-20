pub mod constraint;

mod column;
mod extension;
mod function;
mod relation;
mod schema;
mod trigger;
mod types;

pub use column::*;
pub use constraint::{Constraint, Index, constraints, indexes};
pub use extension::*;
pub use function::*;
pub use relation::*;
pub use schema::*;
pub use trigger::*;
pub use types::*;

#[derive(Clone, Debug, Eq, PartialEq, elephantry_derive::Enum)]
pub enum Kind {
    #[elephantry(value = "r")]
    OrdinaryTable,
    #[elephantry(value = "i")]
    Index,
    #[elephantry(value = "S")]
    Sequence,
    #[elephantry(value = "t")]
    ToastTable,
    #[elephantry(value = "v")]
    View,
    #[elephantry(value = "m")]
    MaterializedView,
    #[elephantry(value = "c")]
    CompositeType,
    #[elephantry(value = "f")]
    ForeignTable,
    #[elephantry(value = "p")]
    PartitionedTable,
    #[elephantry(value = "I")]
    PartitionedIndex,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Kind::OrdinaryTable => "table",
            Kind::Index => "index",
            Kind::Sequence => "sequence",
            Kind::ToastTable => "TOAST table",
            Kind::View => "view",
            Kind::MaterializedView => "materialized view",
            Kind::CompositeType => "composite type",
            Kind::ForeignTable => "foreign table",
            Kind::PartitionedTable => "partitioned table",
            Kind::PartitionedIndex => "partitioned index",
        };

        f.write_str(s)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, elephantry_derive::Enum)]
pub enum Persistence {
    #[elephantry(value = "p")]
    Permanent,
    #[elephantry(value = "u")]
    Unlogged,
    #[elephantry(value = "t")]
    Temporary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, elephantry_derive::Enum)]
pub enum Type {
    #[elephantry(value = "b")]
    Base,
    #[elephantry(value = "c")]
    Composite,
    #[elephantry(value = "d")]
    Domain,
    #[elephantry(value = "e")]
    Enum,
    #[elephantry(value = "p")]
    Pseudo,
    #[elephantry(value = "r")]
    Range,
    #[elephantry(value = "m")]
    Multirange,
}

pub(crate) fn schema_oid(connection: &crate::Connection, name: &str) -> crate::Result<i32> {
    connection
        .query_one::<i32>(
            "
select
    s.oid as oid
from
    pg_catalog.pg_namespace s
where s.nspname = $*
    ",
            &[&name],
        )
        .map_err(|_| crate::Error::Inspect(format!("Unknow schema {name}")))
}
