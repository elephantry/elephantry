pub trait RowStructure
{
    fn relation() -> &'static str;
    fn primary_key() -> &'static [&'static str];
    fn definition() -> std::collections::HashMap<&'static str, crate::Row>;
}
