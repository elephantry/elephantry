#[derive(Clone, Debug, elephantry_derive::Entity)]
#[elephantry(internal)]
pub struct Column {
    #[elephantry(default)]
    pub is_primary: bool,
    pub name: String,
    pub oid: crate::pq::Oid,
    pub ty: String,
    pub default: Option<String>,
    pub is_notnull: bool,
    pub comment: Option<String>,
}

/**
 * Retreive columns of the `schema.relation` relation.
 */
pub fn relation(
    connection: &crate::Connection,
    schema: &str,
    relation: &str,
) -> crate::Result<Vec<crate::inspect::Column>> {
    let oid = connection
        .query_one::<i32>(
            r#"
select c.oid as oid
    from
        pg_catalog.pg_class c
            left join pg_catalog.pg_namespace n on n.oid = c.relnamespace
    where n.nspname = $1
        and c.relname = $2
    "#,
            &[&schema, &relation],
        )
        .map_err(|_| crate::Error::Inspect(format!("Unknow relation {schema}.{relation}")))?;

    connection
        .query(
            r#"
select
    att.attnum = any(ind.indkey) as "is_primary",
    att.attname as "name",
    typ.oid as "oid",
    case
      when att.attlen < 0 and att.atttypmod > 0 then format('%s(%s)', typ.typname, att.atttypmod - 4)
      when name.nspname != 'pg_catalog' then format('%s.%s', name.nspname, typ.typname)
      else typ.typname
    end as "ty",
    pg_catalog.pg_get_expr(def.adbin, def.adrelid) as "default",
    att.attnotnull as "is_notnull",
    dsc.description as "comment"
from
  pg_catalog.pg_attribute att
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
    and att.attrelid = $*
order by
    att.attnum
"#,
            &[&oid],
        )
        .map(|x| x.collect())
}
