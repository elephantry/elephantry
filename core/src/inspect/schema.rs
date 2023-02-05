#[derive(Clone, Debug, elephantry_derive::Entity)]
#[elephantry(internal)]
pub struct Schema {
    pub name: String,
    pub oid: crate::pq::Oid,
    pub relations: u32,
    #[elephantry(default)]
    pub comment: String,
}

/**
 * Retreive schemas of the connected database.
 */
pub fn database(connection: &crate::Connection) -> crate::Result<Vec<crate::inspect::Schema>> {
    connection
        .query(
            r#"
select
    n.nspname     as "name",
    n.oid         as "oid",
    count(c)      as "relations",
    d.description as "comment"
from pg_catalog.pg_namespace n
    left join pg_catalog.pg_description d on n.oid = d.objoid
    left join pg_catalog.pg_class c on
        c.relnamespace = n.oid and c.relkind = any($*)
where n.nspname !~ '^pg' and n.nspname <> 'information_schema'
group by 1, 2, 4
order by 1;
"#,
            &[&vec![super::Kind::OrdinaryTable, super::Kind::View]],
        )
        .map(|x| x.collect())
}
