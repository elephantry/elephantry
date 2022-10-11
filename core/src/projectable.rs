pub trait Projectable {
    /** Get relation name. */
    fn relation() -> &'static str;
    /** Get the list for columns. */
    fn columns() -> &'static [&'static str];
}
