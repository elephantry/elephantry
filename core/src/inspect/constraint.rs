#[derive(Clone, Debug, Eq, PartialEq, elephantry_derive::Enum)]
pub enum Type {
    #[elephantry(value = "c")]
    Check,
    #[elephantry(value = "f")]
    Foreign,
    #[elephantry(value = "n")]
    NotNull,
    #[elephantry(value = "p")]
    PrimaryKey,
    #[elephantry(value = "t")]
    Trigger,
    #[elephantry(value = "u")]
    Unique,
    #[elephantry(value = "x")]
    Exclusion,
}

#[derive(Clone, Debug, Eq, PartialEq, elephantry_derive::Entity, elephantry_derive::Composite)]
pub struct Constraint {
    pub oid: crate::pq::Oid,
    pub ty: Type,
    pub name: String,
    pub definition: String,
    pub columns: Vec<String>,
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
select c.oid, c.contype as ty, c.conname as name, pg_get_constraintdef(c.oid) as definition, array_remove(array_agg(a.attname), null) as columns
    from pg_catalog.pg_constraint c
    left join pg_catalog.pg_attribute a
        on a.attnum = any(c.conkey) and a.attrelid = c.conrelid
    where c.contypid = $1
        or c.conrelid = $1
    group by 1, 2, 3, 4;
",
            &[&oid],
        )
        .map(Iterator::collect)
}

#[derive(Clone, Debug, Eq, PartialEq, elephantry_derive::Entity, elephantry_derive::Composite)]
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
            "
select i.indexrelid as oid, c.relname as name, pg_get_indexdef(c.oid) as definition, array_remove(array_agg(a.attname), null) as columns
    from pg_index i
    join pg_class c on c.oid = i.indexrelid
    left join pg_constraint x on x.conindid = c.oid
    left join pg_catalog.pg_attribute a
        on a.attnum = any(i.indkey) and a.attrelid = i.indrelid
    where i.indrelid = $1
        and x.oid is null
    group by 1, 2, 3;
",
            &[&relation.oid],
        )
        .map(Iterator::collect)
}
