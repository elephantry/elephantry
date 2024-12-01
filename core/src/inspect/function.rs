#[derive(Clone, Debug, PartialEq, Eq, elephantry_derive::Entity)]
pub struct Function {
    pub oid: crate::pq::Oid,
    pub name: String,
    pub language: String,
    pub definition: String,
    pub arguments: String,
    pub return_type: String,
}

pub fn functions(connection: &crate::Connection, schema: &str) -> crate::Result<Vec<Function>> {
    connection
        .query(
            "
select p.oid,
       p.proname as name,
       l.lanname as language,
       case when l.lanname = 'internal' then p.prosrc
            else pg_get_functiondef(p.oid)
            end as definition,
       pg_get_function_arguments(p.oid) as arguments,
       t.typname as return_type
from pg_proc p
left join pg_depend d on p.oid = d.objid and d.deptype = 'e'
left join pg_extension e on e.oid = d.refobjid
left join pg_namespace n on p.pronamespace = n.oid
left join pg_language l on p.prolang = l.oid
left join pg_type t on t.oid = p.prorettype
where n.nspname = $*
    and e.oid is null
order by name;
",
            &[&schema],
        )
        .map(Iterator::collect)
}
