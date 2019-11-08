#[derive(Clone)]
pub struct Row
{
    pub content: &'static str,
    pub ty: postgres::types::Type,
}
