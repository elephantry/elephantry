#[derive(Clone, Debug, elephantry_derive::Entity)]
#[elephantry(internal)]
pub struct Relation {
    pub name: String,
    #[deprecated(since = "3.1.0", note = "Use `kind` field instead")]
    pub ty: String,
    pub persistence: super::Persistence,
    pub kind: super::Kind,
    pub oid: crate::pq::Oid,
    pub comment: Option<String>,
    pub definition: Option<String>,
}

/**
 * Retreive relations (ie: tables, views, …) of `schema`.
 */
pub fn schema(
    connection: &crate::Connection,
    schema: &str,
) -> crate::Result<Vec<crate::inspect::Relation>> {
    let oid = super::schema_oid(connection, schema)?;

    connection
        .query(
            r#"
select
    cl.relname        as "name",
    cl.relpersistence as "persistence",
    case
        when cl.relkind = 'r' then 'table'
        when cl.relkind = 'v' then 'view'
        when cl.relkind = 'm' then 'materialized view'
        when cl.relkind = 'f' then 'foreign table'
        else 'other'
    end               as "ty",
    cl.relkind        as "kind",
    cl.oid            as "oid",
    des.description   as "comment",
    v.definition      as "definition"
from
    pg_catalog.pg_class cl
        left join pg_catalog.pg_description des on
            cl.oid = des.objoid and des.objsubid = 0
        left join pg_catalog.pg_views v on
            v.viewname = cl.relname and v.schemaname = $*
where relkind = any($*)
and cl.relnamespace = $*
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
