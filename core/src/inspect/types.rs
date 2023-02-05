#[derive(Clone, Debug, elephantry_derive::Entity)]
#[elephantry(internal)]
pub struct Enum {
    pub name: String,
    pub elements: Vec<String>,
    pub description: Option<String>,
}

/**
 * Retreive enumeration for `schema`.
 */
pub fn enums(
    connection: &crate::Connection,
    schema: &str,
) -> crate::Result<Vec<crate::inspect::Enum>> {
    types(connection, schema, super::Type::Enum).map(|x| x.collect())
}

#[derive(Clone, Debug, elephantry_derive::Entity)]
#[elephantry(internal)]
pub struct Domain {
    pub name: String,
    pub description: Option<String>,
}

/**
 * Retreive domain for `schema`.
 */
pub fn domains(
    connection: &crate::Connection,
    schema: &str,
) -> crate::Result<Vec<crate::inspect::Domain>> {
    types(connection, schema, super::Type::Domain).map(|x| x.collect())
}

#[derive(Clone, Debug, elephantry_derive::Entity)]
#[elephantry(internal)]
pub struct Composite {
    pub name: String,
    #[elephantry(default)]
    pub fields: Vec<(String, crate::pq::Type)>,
    pub description: Option<String>,
}

/**
 * Retreive composite type for `schema`.
 */
pub fn composites(
    connection: &crate::Connection,
    schema: &str,
) -> crate::Result<Vec<crate::inspect::Composite>> {
    let mut composites = types(connection, schema, super::Type::Composite)?
        .collect::<Vec<crate::inspect::Composite>>();

    for composite in &mut composites {
        composite.fields = crate::inspect::composite_fields(connection, &composite.name)?;
    }

    Ok(composites)
}

pub(crate) fn composite_fields(
    connection: &crate::Connection,
    composite: &str,
) -> crate::Result<Vec<(String, crate::pq::Type)>> {
    connection
        .query(
            r#"
select row(a.attname, pg_catalog.format_type(a.atttypid, a.atttypmod))
    from pg_catalog.pg_attribute a
    join pg_catalog.pg_class c
        on a.attrelid = c.oid
            and c.relname = $*
    where a.attnum > 0 and not a.attisdropped
    order by a.attnum;
        "#,
            &[&composite],
        )
        .map(|x| x.collect())
}

pub(crate) fn types<E: crate::Entity>(
    connection: &crate::Connection,
    schema: &str,
    typtype: super::Type,
) -> crate::Result<crate::Rows<E>> {
    super::schema_oid(connection, schema)?;

    connection
        .query(
            r#"
select pg_catalog.format_type(t.oid, null) as "name",
    array(
        select e.enumlabel
        from pg_catalog.pg_enum e
        where e.enumtypid = t.oid
        order by e.enumsortorder
    ) as "elements",
    pg_catalog.obj_description(t.oid, 'pg_type') as "description"
from pg_catalog.pg_type t
    left join pg_catalog.pg_namespace n on n.oid = t.typnamespace
where t.typtype = $*
    and n.nspname = $*
    and (t.typrelid = 0 or (select c.relkind = 'c' from pg_catalog.pg_class c where c.oid = t.typrelid))
    and not exists(select 1 from pg_catalog.pg_type el where el.oid = t.typelem and el.typarray = t.oid)
    and n.nspname <> 'pg_catalog'
    and n.nspname <> 'information_schema'
    and pg_catalog.pg_type_is_visible(t.oid)
order by 1;
    "#,
            &[&typtype, &schema],
        )
}
