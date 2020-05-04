/*
 * https://github.com/postgres/postgres/blob/REL_12_0/src/include/utils/array.h
 */

use byteorder::ReadBytesExt;
use std::convert::TryInto;

#[derive(Clone, Debug)]
pub struct Array<T> {
    ndim: usize,
    elemtype: crate::pq::Type,
    has_nulls: bool,
    dimensions: Vec<i32>,
    lower_bounds: Vec<i32>,
    data: Vec<u8>,
    maker: std::marker::PhantomData<T>,
}

impl<T: crate::FromSql> Iterator for Array<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        use bytes::Buf;

        if self.data.is_empty() {
            return None;
        }

        let mut buf = &self.data.clone()[..];

        let mut len = buf.get_u32() as usize;
        let value = if len == 0xFFFF_FFFF {
            len = 0;
            None
        } else {
            Some(&buf[..len])
        };
        self.data = buf[len..].to_vec();

        Some(T::from_sql(&self.elemtype, crate::pq::Format::Binary, value).unwrap())
    }
}

impl<T: crate::FromSql> crate::FromSql for Array<T> {
    fn from_text(_ty: &crate::pq::Type, _raw: Option<&str>) -> crate::Result<Self> {
        todo!()
    }

    fn from_binary(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut data = raw.unwrap();

        let ndim = data.read_i32::<byteorder::BigEndian>()?;
        if ndim < 0 {
            panic!("Invalid array");
        }

        let has_nulls = data.read_i32::<byteorder::BigEndian>()? != 0;

        let elemtype: crate::pq::Type = data.read_u32::<byteorder::BigEndian>()?
            .try_into()
            .unwrap();

        let mut dimensions = Vec::new();
        let mut lower_bounds = Vec::new();

        for _ in 0..ndim {
            let dimension = data.read_i32::<byteorder::BigEndian>()?;
            dimensions.push(dimension);

            let lower_bound = data.read_i32::<byteorder::BigEndian>()?;
            lower_bounds.push(lower_bound);
        }

        let array = Self {
            ndim: ndim as usize,
            elemtype,
            has_nulls,
            dimensions,
            lower_bounds,
            data: data.to_vec(),
            maker: std::marker::PhantomData,
        };

        Ok(array)
    }
}

impl<T: crate::FromSql + Clone> Into<Vec<T>> for Array<T> {
    fn into(self) -> Vec<T> {
        if self.ndim > 1 {
            panic!("Unable to transform {} dimension array as vector", self.ndim);
        }

        self.collect()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn text_vec() {
        let loxo = crate::test::new_conn();
        let array: Vec<Vec<i32>> = loxo.execute("SELECT '{1, 2}'::int4[]")
            .unwrap()
            .map(|x| x.nth(0))
            .collect();

        assert_eq!(array, vec![vec![1, 2]]);
    }

    #[test]
    fn bin_vec() {
        let loxo = crate::test::new_conn();
        let results: Vec<HashMap<String, Vec<i32>>> = loxo.query("SELECT '{1, 2}'::int4[] as n", &[])
            .unwrap()
            .collect();

        assert_eq!(results[0].get("n"), Some(&vec![1, 2]));
    }

    #[test]
    fn bin_array_str() {
        let loxo = crate::test::new_conn();
        let results: Vec<HashMap<String, Vec<Option<String>>>> = loxo.query("SELECT '{null, str}'::text[] as n", &[])
            .unwrap()
            .collect();

        assert_eq!(results[0].get("n"), Some(&vec![None, Some("str".to_string())]));
    }
}
