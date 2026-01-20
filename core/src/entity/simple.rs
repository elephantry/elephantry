/**
 * Naive entity trait implementation for type who impl `ToSql` and `FromSql`.
 */
pub trait Simple: crate::ToSql + crate::FromSql {}

impl Simple for () {}
impl Simple for bool {}
impl Simple for char {}
impl Simple for f32 {}
impl Simple for f64 {}
impl Simple for i16 {}
impl Simple for i32 {}
impl Simple for i64 {}
impl Simple for u16 {}
impl Simple for u32 {}
impl Simple for String {}

impl<T: Simple> Simple for Option<T> {}

impl<T: Simple + Clone> Simple for Vec<T> {}

impl<T: Simple> crate::Entity for T {
    fn from(tuple: &crate::Tuple<'_>) -> T {
        tuple.nth(0)
    }

    fn get(&self, _: &str) -> Option<&dyn crate::ToSql> {
        Some(self)
    }
}
