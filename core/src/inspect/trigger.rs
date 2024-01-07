#[derive(Clone, Debug, PartialEq, Eq, elephantry_derive::Entity)]
#[elephantry(internal)]
pub struct Trigger {
    pub action: String,
    pub event: String,
    pub name: String,
    pub orientation: String,
    pub table: String,
    pub timing: String,
}

pub fn triggers(connection: &crate::Connection, schema: &str) -> crate::Result<Vec<Trigger>> {
    connection
        .query(
            r#"
select t.trigger_name as name,
    t.event_manipulation as event,
    t.action_statement as action,
    t.action_timing as timing,
    event_object_table as table,
    t.action_orientation orientation
from information_schema.triggers t
where t.trigger_schema = $*
order by t.trigger_name;
"#,
            &[&schema],
        )
        .map(Iterator::collect)
}
