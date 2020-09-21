pub trait Structure {
    fn relation() -> &'static str;
    fn primary_key() -> &'static [&'static str];
    fn columns() -> &'static [&'static str];
}
