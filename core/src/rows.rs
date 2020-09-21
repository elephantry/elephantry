#[derive(Debug)]
pub struct Rows<E: crate::Entity> {
    result: crate::pq::Result,
    marker: std::marker::PhantomData<E>,
}

impl<E: crate::Entity> Rows<E> {
    pub fn get(&self, n: usize) -> E {
        self.try_get(n).unwrap_or_else(|| {
            panic!("Unable to retreive row: the len is {} but the index is {}", self.len(), n)
        })
    }

    pub fn try_get(&self, n: usize) -> Option<E> {
        self.result.try_get(n).map(|x| E::from(&x))
    }
}

impl<E: crate::Entity> std::iter::Iterator for Rows<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        (&self.result).next().map(|x| E::from(&x))
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

#[cfg(feature = "serde-support")]
impl<E: crate::Entity + serde::Serialize> serde::Serialize for Rows<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(self.len()))?;

        for x in 0..self.result.len() {
            let row = self.result.get(x);

            seq.serialize_element(&E::from(&row))?;
        }

        seq.end()
    }
}
