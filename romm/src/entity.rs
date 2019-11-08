pub trait Entity: Clone
{
    fn from(data: &std::collections::HashMap<&'static str, (postgres::types::Type, Vec<u8>)>) -> Self;
}
