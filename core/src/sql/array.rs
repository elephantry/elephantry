use crate::pq::ToArray;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::convert::TryInto;

/**
 * Rust type for [array](https://www.postgresql.org/docs/current/arrays.html).
 */
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Array<T> {
    ndim: usize,
    elemtype: crate::pq::Type,
    has_nulls: bool,
    dimensions: Vec<i32>,
    lower_bounds: Vec<i32>,
    data: Vec<T>,
}

impl<T: crate::FromSql> Array<T> {
    fn shift_idx(&self, indices: &[i32]) -> usize {
        if self.dimensions.len() != indices.len() {
            panic!();
        }

        let mut acc = 0;
        let mut stride = 1;

        for (x, idx) in indices.iter().enumerate().rev() {
            let dimension = self.dimensions[x];
            let lower_bounds = self.lower_bounds[x] - 1;

            let shifted = idx - lower_bounds;

            acc += shifted * stride;
            stride *= dimension;
        }

        acc as usize
    }
}

impl<T: crate::FromSql> Iterator for Array<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }
}

macro_rules! tuple_impls {
    ($($name:ident : $t:ty),+) => {
        impl<T: crate::FromSql> std::ops::Index<($($t,)+)> for Array<T> {
            type Output = T;

            fn index(&self, ($($name,)+): ($($t,)+)) -> &Self::Output {
                let index = self.shift_idx(&[$($name,)+]);

                &self.data[index]
            }
        }
    }
}

tuple_impls!(a: i32);
tuple_impls!(a: i32, b: i32);
tuple_impls!(a: i32, b: i32, c: i32);
tuple_impls!(a: i32, b: i32, c: i32, d: i32);
tuple_impls!(a: i32, b: i32, c: i32, d: i32, e: i32);
tuple_impls!(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32);
tuple_impls!(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32);
tuple_impls!(
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
    f: i32,
    g: i32,
    h: i32
);
tuple_impls!(
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
    f: i32,
    g: i32,
    h: i32,
    i: i32
);

impl<T: crate::FromSql> std::ops::Index<i32> for Array<T> {
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        self.index((index,))
    }
}

impl<T: crate::FromSql> crate::FromSql for Array<T> {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/arrayfuncs.c#L1012
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let raw = crate::not_null(raw)?;

        let mut has_nulls = false;
        let mut dimensions = Vec::new();
        let mut lower_bounds = Vec::new();
        let mut data = Vec::new();

        let elemtype = ty.elementype();

        let mut current = String::new();
        let mut it = raw.chars().peekable();

        #[allow(clippy::while_let_on_iterator)]
        while let Some(c) = it.next() {
            match c {
                '[' => (),
                ':' => {
                    lower_bounds.push(current.parse()?);
                    current = String::new();
                }
                ']' => {
                    let lower_bound = lower_bounds.last().unwrap_or(&0);
                    dimensions.push(current.parse::<i32>()? - lower_bound + 1);

                    current = String::new();
                }
                '0'..='9' | '-' => current.push(c),
                _ => break,
            }
        }

        #[allow(clippy::while_let_on_iterator)]
        while let Some(c) = it.next() {
            match c {
                '{' => current = String::new(),
                ',' | '}' => {
                    if !current.is_empty() {
                        let value = if current.eq_ignore_ascii_case("null") {
                            has_nulls = true;
                            None
                        } else {
                            Some(current.as_str())
                        };
                        data.push(T::from_text(&elemtype, value)?);
                        current = String::new();
                    }
                }
                _ => current.push(c),
            }
        }

        let array = Self {
            ndim: dimensions.len(),
            elemtype,
            has_nulls,
            dimensions,
            lower_bounds,
            data,
        };

        Ok(array)
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/arrayfuncs.c#L1547
     */
    fn from_binary(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        use std::io::Read;

        let mut raw = crate::not_null(raw)?;

        let ndim = raw.read_i32::<byteorder::BigEndian>()?;
        if ndim < 0 {
            panic!("Invalid array");
        }

        let has_nulls = raw.read_i32::<byteorder::BigEndian>()? != 0;

        let oid = raw.read_u32::<byteorder::BigEndian>()?;
        let elemtype: crate::pq::Type = oid.try_into().unwrap_or(crate::pq::Type {
            oid,
            descr: "Custom type",
            name: "custom",
            kind: libpq::types::Kind::Composite,
        });

        let mut dimensions = Vec::new();
        let mut lower_bounds = Vec::new();

        for _ in 0..ndim {
            let dimension = raw.read_i32::<byteorder::BigEndian>()?;
            dimensions.push(dimension);

            let lower_bound = raw.read_i32::<byteorder::BigEndian>()?;
            lower_bounds.push(lower_bound);
        }

        let mut data = Vec::new();

        while !raw.is_empty() {
            let len = raw.read_u32::<byteorder::BigEndian>()? as usize;

            let value = if len == 0xFFFF_FFFF {
                None
            } else {
                let mut buf = vec![0; len];
                raw.read_exact(buf.as_mut_slice())?;

                Some(buf)
            };

            let element = T::from_sql(&elemtype, crate::pq::Format::Binary, value.as_deref())?;

            data.push(element);
        }

        let array = Self {
            ndim: ndim as usize,
            elemtype,
            has_nulls,
            dimensions,
            lower_bounds,
            data,
        };

        Ok(array)
    }
}

impl<T: crate::ToSql> crate::ToSql for Array<T> {
    fn ty(&self) -> crate::pq::Type {
        self.elemtype.to_array()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/arrayfuncs.c#L172
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        if self.data.is_empty() {
            return Ok(Some(b"{}\0".to_vec()));
        }

        let mut data = Vec::new();

        let need_dims = self
            .lower_bounds
            .iter()
            .fold(false, |acc, x| acc | (*x != 1));

        if need_dims {
            for (dim, lb) in self.dimensions.iter().zip(&self.lower_bounds) {
                data.extend(format!("[{}:{}]", lb, lb + dim - 1).as_bytes());
            }

            data.push(b'=');
        }

        data.push(b'{');

        let mut indx = vec![0; self.ndim];
        let mut j = 0;
        let mut k = 0;

        'outer: loop {
            data.resize(data.len() + self.ndim - 1 - j as usize, b'{');

            let mut element = self.data[k]
                .to_text()?
                .unwrap_or_else(|| b"null\0".to_vec());
            element.pop(); // removes \0

            data.append(&mut element);
            k += 1;

            for i in (0..self.ndim).rev() {
                j = i as i32;
                indx[i] += 1;

                if indx[i] < self.dimensions[i] {
                    data.push(b',');
                    break;
                } else {
                    indx[i] = 0;
                    data.push(b'}');
                }

                if i == 0 {
                    break 'outer;
                }
            }
        }

        data.push(b'\0');

        Ok(Some(data))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/arrayfuncs.c#L1267
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();

        buf.write_i32::<byteorder::BigEndian>(self.ndim as i32)?;
        buf.write_i32::<byteorder::BigEndian>(self.has_nulls as i32)?;
        buf.write_i32::<byteorder::BigEndian>(self.ty().elementype().oid as i32)?;

        for x in 0..self.ndim {
            buf.write_i32::<byteorder::BigEndian>(self.dimensions[x])?;
            buf.write_i32::<byteorder::BigEndian>(self.lower_bounds[x])?;
        }

        for d in &self.data {
            if let Some(raw) = d.to_binary()? {
                buf.write_i32::<byteorder::BigEndian>(raw.len() as i32)?;
                buf.extend(&raw);
            } else {
                buf.write_i32::<byteorder::BigEndian>(-1)?;
            }
        }

        Ok(Some(buf))
    }
}

impl<T: crate::FromSql> From<Array<T>> for Vec<T> {
    fn from(array: Array<T>) -> Self {
        if array.ndim > 1 {
            panic!(
                "Unable to transform {} dimension array as vector",
                array.ndim
            );
        }

        array.collect()
    }
}

impl<T: crate::ToSql + Clone> From<&Vec<T>> for Array<T> {
    fn from(data: &Vec<T>) -> Self {
        use crate::ToSql;

        Self {
            ndim: 1,
            elemtype: data.ty(),
            dimensions: vec![data.len() as i32],
            lower_bounds: vec![1],
            has_nulls: false,
            data: data.clone(),
        }
    }
}

impl<T: crate::ToSql + Clone> crate::ToSql for Vec<T> {
    fn ty(&self) -> crate::pq::Type {
        match self.get(0) {
            Some(data) => data.ty().to_array(),
            None => crate::pq::types::UNKNOWN,
        }
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        crate::sql::Array::from(self).to_text()
    }

    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        crate::sql::Array::from(self).to_binary()
    }
}

impl<T: crate::FromSql> crate::FromSql for Vec<T> {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        Ok(crate::Array::from_text(ty, raw)?.into())
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        Ok(crate::Array::from_binary(ty, raw)?.into())
    }
}

#[cfg(test)]
mod test {
    use crate::ToSql;

    #[test]
    fn array_from_vec() {
        let array = crate::Array::from(&vec![1, 2, 3]);

        assert_eq!(array.ndim, 1);
        assert_eq!(array[2], 3);
    }

    #[test]
    fn vec_to_text() {
        let vec = vec![1, 2, 3];

        assert_eq!(vec.to_text().unwrap(), Some(b"{1,2,3}\0".to_vec()));
    }

    #[test]
    fn empty_vec() {
        let vec = Vec::<String>::new();

        assert_eq!(vec.to_text().unwrap(), Some(b"{}\0".to_vec()));
    }

    #[test]
    fn array_index() {
        let array = crate::Array {
            ndim: 2,
            elemtype: crate::pq::types::INT8,
            has_nulls: false,
            dimensions: vec![3, 2],
            lower_bounds: vec![1, 1],
            data: vec![1, 2, 3, 4, 5, 6],
        };

        assert_eq!(array[(2, 1)], 6);
    }

    crate::sql_test!(_int4, Vec<i32>, [("'{1, 2}'", vec![1, 2]),]);

    crate::sql_test!(
        _int8,
        crate::Array<i64>,
        [(
            "'[1:1][-2:-1][3:5]={{{1,2,3},{4,5,6}}}'",
            crate::Array {
                ndim: 3,
                elemtype: crate::pq::types::INT8,
                has_nulls: false,
                dimensions: vec![1, 2, 3],
                lower_bounds: vec![1, -2, 3],
                data: vec![1, 2, 3, 4, 5, 6],
            }
        )]
    );

    crate::sql_test!(
        _varchar,
        Vec<Option<String>>,
        [("'{str, null, \'\'null\'\'}'", vec![Some("str".to_string()), None, Some("null".to_string())])]
    );
}
