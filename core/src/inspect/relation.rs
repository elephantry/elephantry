#[derive(Clone, Debug, Eq, PartialEq, elephantry_derive::Entity)]
pub struct Relation {
    pub name: String,
    pub persistence: super::Persistence,
    pub kind: super::Kind,
    pub oid: crate::pq::Oid,
    pub comment: Option<String>,
    pub definition: Option<String>,
    pub schema: String,
}

/**
 * Retreive relations (ie: tables, views, …) of `schema`.
 */
pub fn schema(connection: &crate::Connection, schema: &str) -> crate::Result<Vec<Relation>> {
    let oid = super::schema_oid(connection, schema)?;

    connection
        .query(
            r#"
select
    cl.relname        as "name",
    cl.relpersistence as "persistence",
    cl.relkind        as "kind",
    cl.oid            as "oid",
    des.description   as "comment",
    case
        when cl.relkind = 'v' then v.definition
        when cl.relkind = 'm' then mv.definition
        else null
    end               as "definition",
    $1                as "schema"
from
    pg_catalog.pg_class cl
        left join pg_catalog.pg_description des on
            cl.oid = des.objoid and des.objsubid = 0
        left join pg_catalog.pg_views v on
            v.viewname = cl.relname and v.schemaname = $1
        left join pg_catalog.pg_matviews mv on
            mv.matviewname = cl.relname and mv.schemaname = $1
where relkind = any($2)
and cl.relnamespace = $3
order by name asc;
"#,
            &[
                &schema,
                &vec![
                    super::Kind::OrdinaryTable,
                    super::Kind::View,
                    super::Kind::MaterializedView,
                    super::Kind::ForeignTable,
                ],
                &oid,
            ],
        )
        .map(Iterator::collect)
}
