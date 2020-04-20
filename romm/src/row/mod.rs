mod structure;

pub use structure::*;

#[derive(Clone, Debug)]
pub struct Row
{
    pub content: &'static str,
    pub ty: crate::pq::Type,
}
