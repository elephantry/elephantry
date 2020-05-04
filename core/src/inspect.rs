#[derive(Clone, Debug, elephantry_derive::Entity)]
#[entity(internal)]
pub struct Schema {
    pub name: String,
    pub oid: crate::pq::Oid,
    pub relations: String,
    pub comment: String,
}

pub fn database(connection: &crate::Connection) -> Vec<Schema> {
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
        c.relnamespace = n.oid and c.relkind in ('r', 'v')
where n.nspname !~ '^pg' and n.nspname <> 'information_schema'
group by 1, 2, 4
order by 1;
"#,
            &[],
        )
        .unwrap()
        .collect()
}

#[derive(Clone, Debug, elephantry_derive::Entity)]
#[entity(internal)]
pub struct Relation {
    pub name: String,
    pub ty: String,
    pub oid: crate::pq::Oid,
    pub comment: Option<String>,
}

pub fn schema(connection: &crate::Connection, schema: &str) -> Vec<Relation> {
    connection
        .query(
            r#"
with schema as(
    select
        s.oid as oid
    from
        pg_catalog.pg_namespace s
    where s.nspname = $1
)
select
    cl.relname      as "name",
    case
        when cl.relkind = 'r' then 'table'
        when cl.relkind = 'v' then 'view'
        when cl.relkind = 'm' then 'materialized view'
        when cl.relkind = 'f' then 'foreign table'
        else 'other'
    end             as "ty",
    cl.oid          as "oid",
    des.description as "comment"
from
    pg_catalog.pg_class cl
        left join pg_catalog.pg_description des on
            cl.oid = des.objoid and des.objsubid = 0
join schema on schema.oid = cl.relnamespace
where relkind in ('r', 'v', 'm', 'f')
order by name asc;
"#,
            &[&schema],
        )
        .unwrap()
        .collect()
}

#[derive(Clone, Debug, elephantry_derive::Entity)]
#[entity(internal)]
pub struct Column {
    pub is_primary: bool,
    pub name: String,
    pub oid: crate::pq::Oid,
    pub ty: String,
    pub default: Option<String>,
    pub is_notnull: bool,
    pub comment: Option<String>,
}

pub fn relation(connection: &crate::Connection, schema: &str, relation: &str) -> Vec<Column> {
    connection
        .query(
            r#"
with relation as(
    select
    c.oid as oid
    from
        pg_catalog.pg_class c
            left join pg_catalog.pg_namespace n on n.oid = c.relnamespace
    where
    n.nspname = $1
    and c.relname = $2
)
select
    att.attnum = any(ind.indkey) as "is_primary",
    att.attname      as "name",
    typ.oid as "oid",
    case
        when name.nspname = 'pg_catalog' then typ.typname
        else format('%s.%s', name.nspname, typ.typname)
    end as "ty",
    pg_catalog.pg_get_expr(def.adbin, def.adrelid) as "default",
    att.attnotnull as "is_notnull",
    dsc.description as "comment"
from
  pg_catalog.pg_attribute att
    join relation on att.attrelid = relation.oid
    join pg_catalog.pg_type  typ  on att.atttypid = typ.oid
    join pg_catalog.pg_class cla  on att.attrelid = cla.oid
    join pg_catalog.pg_namespace clns on cla.relnamespace = clns.oid
    left join pg_catalog.pg_description dsc on cla.oid = dsc.objoid and att.attnum = dsc.objsubid
    left join pg_catalog.pg_attrdef def     on att.attrelid = def.adrelid and att.attnum = def.adnum
    left join pg_catalog.pg_index ind       on cla.oid = ind.indrelid and ind.indisprimary
    left join pg_catalog.pg_namespace name  on typ.typnamespace = name.oid
where
    att.attnum > 0
    and not att.attisdropped
order by
    att.attnum
"#,
            &[&schema, &relation],
        )
        .unwrap()
        .collect()
}
