#[derive(Debug)]
pub struct Rows<E: crate::Entity> {
    result: crate::pq::Result,
    marker: std::marker::PhantomData<E>,
}

impl<E: crate::Entity> std::iter::Iterator for Rows<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        (&self.result).next()
            .map(|x| E::from(&x))
    }
}

impl<E: crate::Entity> From<crate::pq::Result> for Rows<E> {
    fn from(result: crate::pq::Result) -> Self {
        Self {
            result,
            marker: std::marker::PhantomData,
        }
    }
}

impl<E: crate::Entity> std::ops::Deref for Rows<E> {
    type Target = crate::pq::Result;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}
