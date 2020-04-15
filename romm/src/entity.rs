pub trait Entity: Clone
{
    fn from(data: &std::collections::HashMap<&'static str, (postgres::types::Type, Vec<u8>)>) -> Self;
    fn get(&self, field: &str) -> Option<&dyn postgres::types::ToSql>;
}
