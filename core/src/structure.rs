/**
 * Trait to reflect relation structure.
 */
pub trait Structure {
    /** Get relation name. */
    fn relation() -> &'static str;
    /** Get the list of column contitutes the primary key. */
    fn primary_key() -> &'static [&'static str];
    /** Get the list for columns. */
    fn columns() -> &'static [&'static str];
}
