#![allow(unused_variables)]

#[derive(Debug, elephantry_derive::Entity)]
#[entity(internal)]
pub struct Schema {
    pub name: String,
    pub oid: crate::pq::Oid,
    pub relations: String,
    pub comment: String,
}

/**
 * Retreive schemas of the connected database.
 */
#[deprecated(
    since = "1.6.0",
    note = "use crate::v2::inspect::database instead"
)]
#[cfg(not(feature = "v2"))]
pub fn database(connection: &crate::Connection) -> Vec<Schema> {
    crate::v2::inspect::database(&connection).unwrap()
}

/**
 * Retreive schemas of the connected database.
 */
#[deprecated(
    since = "1.7.0",
    note = "use crate::v2::inspect::database instead"
)]
#[cfg(feature = "v2")]
pub fn database(connection: &crate::Connection) -> crate::Result<Vec<Schema>> {
    crate::v2::inspect::database(connection)
}

#[derive(Debug, elephantry_derive::Entity)]
#[entity(internal)]
pub struct Relation {
    pub name: String,
    pub ty: String,
    pub oid: crate::pq::Oid,
    pub comment: Option<String>,
}

/**
 * Retreive relations (ie: tables, views, …) of `schema`.
 */
#[deprecated(
    since = "1.6.0",
    note = "use crate::v2::inspect::schema instead"
)]
#[cfg(not(feature = "v2"))]
pub fn schema(connection: &crate::Connection, schema: &str) -> Vec<Relation> {
    crate::v2::inspect::schema(&connection, schema).unwrap()
}

/**
 * Retreive relations (ie: tables, views, …) of `schema`.
 */
#[deprecated(
    since = "1.7.0",
    note = "use crate::v2::inspect::schema instead"
)]
#[cfg(feature = "v2")]
pub fn schema(
    connection: &crate::Connection,
    schema: &str,
) -> crate::Result<Vec<Relation>> {
    crate::v2::inspect::schema(&connection, schema)
}

#[derive(Debug, elephantry_derive::Entity)]
#[entity(internal)]
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
#[deprecated(
    since = "1.6.0",
    note = "use crate::v2::inspect::relation instead"
)]
#[cfg(not(feature = "v2"))]
pub fn relation(
    connection: &crate::Connection,
    schema: &str,
    relation: &str,
) -> Vec<Column> {
    crate::v2::inspect::relation(connection, schema, relation).unwrap()
}

/**
 * Retreive columns of the `schema.relation` relation.
 */
#[deprecated(
    since = "1.7.0",
    note = "use crate::v2::inspect::relation instead"
)]
#[cfg(feature = "v2")]
pub fn relation(
    connection: &crate::Connection,
    schema: &str,
    relation: &str,
) -> crate::Result<Vec<Column>> {
    crate::v2::inspect::relation(connection, schema, relation)
}

#[derive(Debug, elephantry_derive::Entity)]
#[entity(internal)]
pub struct Enum {
    pub name: String,
    pub elements: Vec<String>,
    pub description: Option<String>,
}

/**
 * Retreive enumeration for `schema`.
 */
#[deprecated(
    since = "1.6.0",
    note = "use crate::v2::inspect::enums instead"
)]
#[cfg(not(feature = "v2"))]
pub fn enums(connection: &crate::Connection, schema: &str) -> Vec<Enum> {
    crate::v2::inspect::enums(connection, schema).unwrap()
}

/**
 * Retreive enumeration for `schema`.
 */
#[deprecated(
    since = "1.6.0",
    note = "use crate::v2::inspect::enums instead"
)]
#[cfg(feature = "v2")]
pub fn enums(
    connection: &crate::Connection,
    schema: &str,
) -> crate::Result<Vec<Enum>> {
    crate::v2::inspect::enums(connection, schema)
}

#[derive(Debug, elephantry_derive::Entity)]
#[entity(internal)]
pub struct Domain {
    pub name: String,
    pub description: Option<String>,
}

/**
 * Retreive domain for `schema`.
 */
#[deprecated(
    since = "1.6.0",
    note = "use crate::v2::inspect::domains instead"
)]
#[cfg(not(feature = "v2"))]
pub fn domains(connection: &crate::Connection, schema: &str) -> Vec<Domain> {
    crate::v2::inspect::domains(connection, schema).unwrap()
}

/**
 * Retreive domain for `schema`.
 */
#[deprecated(
    since = "1.7.0",
    note = "use crate::v2::inspect::domains instead"
)]
#[cfg(feature = "v2")]
pub fn domains(
    connection: &crate::Connection,
    schema: &str,
) -> crate::Result<Vec<Domain>> {
    crate::v2::inspect::domains(connection, schema)
}

#[derive(Debug, elephantry_derive::Entity)]
#[entity(internal)]
pub struct Composite {
    pub name: String,
    #[elephantry(default)]
    pub fields: Vec<(String, String)>,
    pub description: Option<String>,
}

/**
 * Retreive composite type for `schema`.
 */
#[deprecated(
    since = "1.6.0",
    note = "use crate::v2::inspect::composites instead"
)]
#[cfg(not(feature = "v2"))]
pub fn composites(
    connection: &crate::Connection,
    schema: &str,
) -> Vec<Composite> {
    crate::v2::inspect::composites(connection, schema).unwrap()
}

/**
 * Retreive composite type for `schema`.
 */
#[deprecated(
    since = "1.7.0",
    note = "use crate::v2::inspect::composites instead"
)]
#[cfg(feature = "v2")]
pub fn composites(
    connection: &crate::Connection,
    schema: &str,
) -> crate::Result<Vec<Composite>> {
    crate::v2::inspect::composites(connection, schema)
}

pub(crate) fn composite_fields(
    connection: &crate::Connection,
    composite: &str,
) -> crate::Result<Vec<(String, String)>> {
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
    typtype: char,
) -> crate::Result<crate::Rows<E>> {
    schema_oid(connection, schema)?;

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

pub(crate) fn schema_oid(
    connection: &crate::Connection,
    name: &str,
) -> crate::Result<i32> {
    connection
        .query_one::<i32>(
            r#"
select
    s.oid as oid
from
    pg_catalog.pg_namespace s
where s.nspname = $*
    "#,
            &[&name],
        )
        .map_err(|e| {
            #[cfg(feature = "v2")]
            return crate::Error::Inspect(format!("Unknow schema {}", name));

            #[cfg(not(feature = "v2"))]
            return e;
        })
}
