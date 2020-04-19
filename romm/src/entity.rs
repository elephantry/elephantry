pub trait Entity: Clone
{
    fn from(row: &crate::pq::Row) -> Self;
    fn get(&self, field: &str) -> Option<&dyn crate::pq::ToSql>;
}
