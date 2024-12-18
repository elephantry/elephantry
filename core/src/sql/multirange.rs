#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Multirange<R, T>
where
    R: std::ops::RangeBounds<T> + crate::ToSql + crate::FromSql,
    T: std::fmt::Debug,
{
    ranges: Vec<R>,
    _phantodata: std::marker::PhantomData<T>,
}

impl<R, T> Multirange<R, T>
where
    R: std::ops::RangeBounds<T> + crate::ToSql + crate::FromSql,
    T: std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
            _phantodata: std::marker::PhantomData,
        }
    }

    pub fn from(ranges: Vec<R>) -> Self {
        Self {
            ranges,
            _phantodata: std::marker::PhantomData,
        }
    }
}

impl<R, T> Default for Multirange<R, T>
where
    R: std::ops::RangeBounds<T> + crate::ToSql + crate::FromSql,
    T: std::fmt::Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R, T> std::ops::Deref for Multirange<R, T>
where
    R: std::ops::RangeBounds<T> + crate::ToSql + crate::FromSql,
    T: std::fmt::Debug,
{
    type Target = Vec<R>;

    fn deref(&self) -> &Self::Target {
        &self.ranges
    }
}

impl<R, T> std::ops::DerefMut for Multirange<R, T>
where
    R: std::ops::RangeBounds<T> + crate::ToSql + crate::FromSql,
    T: std::fmt::Debug,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ranges
    }
}

impl<R, T> crate::ToSql for Multirange<R, T>
where
    R: std::ops::RangeBounds<T> + crate::ToSql + crate::FromSql,
    T: std::fmt::Debug,
{
    fn ty(&self) -> crate::pq::Type {
        use crate::pq::ToArray;

        let range = match self.ranges.first() {
            Some(range) => range,
            None => return crate::pq::types::UNKNOWN,
        };

        range.ty().to_multi_range()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_14_0/src/backend/utils/adt/multirangetypes.c#L293
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        let mut data = String::from("{");

        for range in &self.ranges {
            let s = range.to_text()?.unwrap_or_default();
            data.push_str(&s);
            data.push(',');
        }

        data.pop(); // removes extra ','
        data.push('}');

        Ok(Some(data))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_14_0/src/backend/utils/adt/multirangetypes.c#L371
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();

        crate::to_sql::write_i32(&mut buf, self.ranges.len() as i32)?;

        for range in &self.ranges {
            if let Some(raw) = range.to_binary()? {
                crate::to_sql::write_i32(&mut buf, raw.len() as i32)?;
                buf.extend(&raw);
            } else {
                return Err(self.error("range element could not be null"));
            }
        }

        Ok(Some(buf))
    }
}

impl<R, T> crate::FromSql for Multirange<R, T>
where
    R: std::ops::RangeBounds<T> + crate::ToSql + crate::FromSql,
    T: std::fmt::Debug,
{
    /*
     * https://github.com/postgres/postgres/blob/REL_14_0/src/backend/utils/adt/multirangetypes.c#L117
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let buf = crate::from_sql::not_null(raw)?;
        let mut ranges = Vec::new();

        let mut range_str = String::new();

        for c in buf.chars() {
            match c {
                '{' | '}' => continue,
                ']' | ')' => {
                    range_str.push(c);
                    let range = R::from_text(ty, Some(&range_str))?;
                    ranges.push(range);
                    range_str = String::new();
                }
                ',' => {
                    if range_str.is_empty() {
                        continue;
                    } else {
                        range_str.push(c);
                    }
                }
                _ => range_str.push(c),
            }
        }

        Ok(Self::from(ranges))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_14_0/src/backend/utils/adt/multirangetypes.c#L331
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        use std::io::Read;

        let mut buf = crate::from_sql::not_null(raw)?;

        let range_count = crate::from_sql::read_i32(&mut buf)?;

        let mut ranges = Vec::with_capacity(range_count as usize);
        for _ in 0..range_count {
            let range_len = crate::from_sql::read_i32(&mut buf)? as usize;
            let mut range_data = vec![0; range_len];
            buf.read_exact(range_data.as_mut_slice())?;

            let range = R::from_binary(ty, Some(&range_data))?;
            ranges.push(range);
        }

        Ok(Self::from(ranges))
    }
}

impl<R, T> crate::entity::Simple for Multirange<R, T>
where
    R: std::ops::RangeBounds<T> + crate::ToSql + crate::FromSql,
    T: std::fmt::Debug,
{
}

#[cfg(test)]
mod test {
    crate::sql_test!(
        int4multirange,
        crate::Multirange<std::ops::Range<i32>, i32>,
        [(
            "'{[0, 10),[11,20)}'",
            crate::Multirange::from(vec![0_i32..10, 11_i32..20]),
        )]
    );
}
