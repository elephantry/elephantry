/**
 * Trait to reflect relation structure.
 */
pub trait Structure: crate::Projectable {
    /** Get the list of column contitutes the primary key. */
    fn primary_key() -> &'static [&'static str];
}
