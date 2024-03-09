pub mod constraint;

mod column;
mod extension;
mod function;
mod relation;
mod schema;
mod trigger;
mod types;

pub use column::*;
pub use constraint::{constraints, indexes, Constraint, Index};
pub use extension::*;
pub use function::*;
pub use relation::*;
pub use schema::*;
pub use trigger::*;
pub use types::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Kind {
    OrdinaryTable,
    Index,
    Sequence,
    ToastTable,
    View,
    MaterializedView,
    CompositeType,
    ForeignTable,
    PartitionedTable,
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

impl crate::ToText for Kind {
    fn to_text(&self) -> crate::Result<String> {
        let s = match self {
            Kind::OrdinaryTable => "r",
            Kind::Index => "i",
            Kind::Sequence => "S",
            Kind::ToastTable => "t",
            Kind::View => "v",
            Kind::MaterializedView => "m",
            Kind::CompositeType => "c",
            Kind::ForeignTable => "f",
            Kind::PartitionedTable => "p",
            Kind::PartitionedIndex => "I",
        };

        Ok(s.to_string())
    }
}

impl crate::FromText for Kind {
    fn from_text(raw: &str) -> crate::Result<Self> {
        let kind = match raw {
            "r" => Self::OrdinaryTable,
            "i" => Self::Index,
            "S" => Self::Sequence,
            "t" => Self::ToastTable,
            "v" => Self::View,
            "m" => Self::MaterializedView,
            "c" => Self::CompositeType,
            "f" => Self::ForeignTable,
            "p" => Self::PartitionedTable,
            "I" => Self::PartitionedIndex,
            _ => return Err(Self::error(raw)),
        };

        Ok(kind)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Persistence {
    Permanent,
    Unlogged,
    Temporary,
}

impl crate::ToText for Persistence {
    fn to_text(&self) -> crate::Result<String> {
        let s = match self {
            Persistence::Permanent => "p",
            Persistence::Unlogged => "u",
            Persistence::Temporary => "t",
        };

        Ok(s.to_string())
    }
}

impl crate::FromText for Persistence {
    fn from_text(raw: &str) -> crate::Result<Self> {
        let persistence = match raw {
            "p" => Self::Permanent,
            "u" => Self::Unlogged,
            "t" => Self::Temporary,
            _ => return Err(Self::error(raw)),
        };

        Ok(persistence)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    Base,
    Composite,
    Domain,
    Enum,
    Pseudo,
    Range,
    Multirange,
}

impl crate::ToText for Type {
    fn to_text(&self) -> crate::Result<String> {
        let s = match self {
            Self::Base => "b",
            Self::Composite => "c",
            Self::Domain => "d",
            Self::Enum => "e",
            Self::Pseudo => "p",
            Self::Range => "r",
            Self::Multirange => "m",
        };

        Ok(s.to_string())
    }
}

impl crate::FromText for Type {
    fn from_text(raw: &str) -> crate::Result<Self> {
        let ty = match raw {
            "b" => Self::Base,
            "c" => Self::Composite,
            "d" => Self::Domain,
            "e" => Self::Enum,
            "p" => Self::Pseudo,
            "r" => Self::Range,
            "m" => Self::Multirange,
            _ => return Err(Self::error(raw)),
        };

        Ok(ty)
    }
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
