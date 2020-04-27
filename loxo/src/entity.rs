pub trait Entity: Clone {
    fn from(tuple: &crate::pq::Tuple<'_>) -> Self;
    fn get(&self, field: &str) -> Option<&dyn crate::pq::ToSql>;
}
