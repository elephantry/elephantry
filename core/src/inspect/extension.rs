#[derive(Clone, Debug, PartialEq, Eq, elephantry_derive::Entity)]
#[elephantry(internal)]
pub struct Extension {
    pub oid: crate::pq::Oid,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

pub fn extensions(connection: &crate::Connection, schema: &str) -> crate::Result<Vec<Extension>> {
    let oid = super::schema_oid(connection, schema)?;

    connection.query("
select e.oid,
    e.extname as name,
    e.extversion as version,
    c.description as description
from pg_extension e
    left join pg_catalog.pg_namespace n on n.oid = e.extnamespace and n.oid = $*
    left join pg_catalog.pg_description c on c.objoid = e.oid and c.classoid = 'pg_catalog.pg_extension'::pg_catalog.regclass
order by e.extname;
", &[&oid])
        .map(Iterator::collect)
}
