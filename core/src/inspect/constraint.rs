#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Check,
    Foreign,
    PrimaryKey,
    Trigger,
    Unique,
    Exclusion,
}

impl crate::ToText for Type {
    fn to_text(&self) -> crate::Result<String> {
        let s = match self {
            Type::Check => "c",
            Type::Foreign => "f",
            Type::PrimaryKey => "p",
            Type::Trigger => "t",
            Type::Unique => "u",
            Type::Exclusion => "x",
        };

        Ok(s.to_string())
    }
}

impl crate::FromText for Type {
    fn from_text(raw: &str) -> crate::Result<Self> {
        let ty = match raw {
            "c" => Self::Check,
            "f" => Self::Foreign,
            "p" => Self::PrimaryKey,
            "t" => Self::Trigger,
            "u" => Self::Unique,
            "x" => Self::Exclusion,
            _ => return Err(Self::error(raw)),
        };

        Ok(ty)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, elephantry_derive::Entity, elephantry_derive::Composite)]
#[elephantry(internal)]
pub struct Constraint {
    pub oid: crate::pq::Oid,
    pub ty: Type,
    pub name: String,
    pub definition: String,
}

/**
 * Retreive constraints.
 */
pub fn constraints(
    connection: &crate::Connection,
    oid: crate::pq::Oid,
) -> crate::Result<Vec<Constraint>> {
    connection
        .query(
            "
select oid, contype as ty, conname as name, pg_get_constraintdef(oid) as definition
    from pg_catalog.pg_constraint
    where contypid = $1
        or conrelid = $1;
",
            &[&oid],
        )
        .map(Iterator::collect)
}

#[derive(Clone, Debug, Eq, PartialEq, elephantry_derive::Entity, elephantry_derive::Composite)]
#[elephantry(internal)]
pub struct Index {
    pub oid: crate::pq::Oid,
    pub name: String,
    pub definition: String,
}

/**
 * Retreive relation indexes.
 */
pub fn indexes(
    connection: &crate::Connection,
    relation: &crate::inspect::Relation,
) -> crate::Result<Vec<Index>> {
    connection
        .query(
            r#"
select i.indexrelid as oid, c.relname as name, pg_get_indexdef(c.oid) as definition
    from pg_index i
    join pg_class c on c.oid = i.indexrelid
    left join pg_constraint x on x.conindid = c.oid
    where i.indrelid = $1
        and x.oid is null;
"#,
            &[&relation.oid],
        )
        .map(Iterator::collect)
}
