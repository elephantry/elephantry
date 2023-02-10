#[derive(Clone, Debug, Eq, PartialEq, elephantry_derive::Entity, elephantry_derive::Composite)]
#[elephantry(internal)]
pub struct Constraint {
    pub oid: crate::pq::Oid,
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
            r#"
select oid, conname as name, pg_get_constraintdef(oid) as definition
    from pg_catalog.pg_constraint
    where contypid = $1
        or conrelid = $1;
"#,
            &[&oid],
        )
        .map(Iterator::collect)
}
