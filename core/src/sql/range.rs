#[repr(u8)]
enum Flags {
    //Empty = 0x01,
    LbInc = 0x02,
    //UbInc = 0x04,
    //LbInf = 0x08,
    //UbInf = 0x10,
}

macro_rules! bound {
    ($range:ident, $bound:ident, $op:ident) => {{
        let bound = match $range.$bound() {
            Included(bound) => bound,
            Excluded(bound) => bound,
            Unbounded => panic!("Unsupported unbounded range"),
        };

        match bound.$op()? {
            Some(bound) => bound,
            None => return Ok(None),
        }
    }};
}

fn ty<R, T>(range: &R) -> crate::pq::Type
where
    R: std::ops::RangeBounds<T>,
    T: crate::ToSql,
{
    use crate::pq::types::*;
    use std::ops::Bound::*;

    let start = match range.start_bound() {
        Included(start) | Excluded(start) => start,
        Unbounded => panic!("Unsupported unbounded range"),
    };

    match start.ty() {
        ANY => ANY_RANGE,
        INT4 => INT4_RANGE,
        INT8 => INT8_RANGE,
        NUMERIC => NUM_RANGE,
        TIMESTAMP => TS_RANGE,
        TIMESTAMPTZ => TSTZ_RANGE,
        DATE => DATE_RANGE,
        _ => UNKNOWN,
    }
}

fn to_text<R, T>(range: &R) -> crate::Result<Option<Vec<u8>>>
where
    R: std::ops::RangeBounds<T>,
    T: crate::ToSql,
{
    use std::ops::Bound::*;

    let start_char = match range.start_bound() {
        Included(_) => b'[',
        Excluded(_) => b'(',
        Unbounded => panic!("Unsupported unbounded range"),
    };

    let mut start = bound!(range, start_bound, to_text);
    start.pop(); // removes \0

    let end_char = match range.end_bound() {
        Included(_) => b']',
        Excluded(_) => b')',
        Unbounded => panic!("Unsupported unbounded range"),
    };

    let mut end = bound!(range, end_bound, to_text);
    end.pop(); // removes \0

    let mut vec = vec![start_char];
    vec.append(&mut start);
    vec.push(b',');
    vec.append(&mut end);
    vec.push(end_char);
    vec.push(b'\0');

    Ok(Some(vec))
}

fn to_binary<R, T>(range: &R) -> crate::Result<Option<Vec<u8>>>
where
    R: std::ops::RangeBounds<T>,
    T: crate::ToSql,
{
    use byteorder::WriteBytesExt;
    use std::ops::Bound::*;

    let mut buf = vec![Flags::LbInc as u8];

    let mut start = bound!(range, start_bound, to_binary);
    buf.write_i32::<byteorder::BigEndian>(start.len() as i32)?;
    buf.append(&mut start);

    let mut end = bound!(range, end_bound, to_binary);
    buf.write_i32::<byteorder::BigEndian>(end.len() as i32)?;
    buf.append(&mut end);

    Ok(Some(buf))
}

impl<T: crate::ToSql> crate::ToSql for std::ops::Range<T> {
    fn ty(&self) -> crate::pq::Type {
        ty(self)
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/rangetypes.c#L123
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        to_text(self)
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/rangetypes.c#L246
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        to_binary(self)
    }
}

impl<T: crate::FromSql> crate::FromSql for std::ops::Range<T> {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/rangetypes.c#L81
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        lazy_static::lazy_static! {
            static ref REGEX: regex::Regex =
                regex::Regex::new(r"[\[\(](?P<start>.?*),(?P<end>.?*)[\]\)]")
                    .unwrap();
        }

        let matches = REGEX.captures(crate::not_null(raw)?).unwrap();

        // tstzrange are quoted
        let start = matches.name("start").map(|x| x.as_str().trim_matches('\"'));
        let end = matches.name("end").map(|x| x.as_str().trim_matches('\"'));

        Ok(std::ops::Range {
            start: T::from_text(ty, start)?,
            end: T::from_text(ty, end)?,
        })
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/rangetypes.c#L163
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        use byteorder::ReadBytesExt;

        let mut buf = crate::from_sql::not_null(raw)?;
        let _flag = buf.read_u8()?;

        let start_bound_len = buf.read_i32::<byteorder::BigEndian>()?;
        let mut start = Vec::new();
        for _ in 0..start_bound_len {
            let b = buf.read_u8()?;

            start.push(b);
        }

        let end_bound_len = buf.read_i32::<byteorder::BigEndian>()?;
        let mut end = Vec::new();
        for _ in 0..end_bound_len {
            let b = buf.read_u8()?;

            end.push(b);
        }

        Ok(std::ops::Range {
            start: T::from_binary(ty, Some(&start))?,
            end: T::from_binary(ty, Some(&end))?,
        })
    }
}

impl<T: crate::ToSql> crate::ToSql for std::ops::RangeInclusive<T> {
    fn ty(&self) -> crate::pq::Type {
        ty(self)
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        to_text(self)
    }

    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        to_text(self)
    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(int4range, std::ops::Range<i32>, [("'[0, 10)'", 0_i32..10)]);

    crate::sql_test!(int8range, std::ops::Range<i64>, [("'[0, 10)'", 0_i64..10)]);

    #[cfg(feature = "numeric")]
    crate::sql_test!(
        numrange,
        std::ops::Range<bigdecimal::BigDecimal>,
        [(
            "'[3900, 20000)'",
            bigdecimal::BigDecimal::from(3_900)..bigdecimal::BigDecimal::from(20_000)
        )]
    );

    #[cfg(feature = "date")]
    crate::sql_test!(
        daterange,
        std::ops::Range<chrono::NaiveDate>,
        [(
            "'[1970-01-01, 2010-01-01)'",
            chrono::NaiveDate::from_ymd(1970, 01, 01)..chrono::NaiveDate::from_ymd(2010, 01, 01)
        )]
    );

    #[cfg(feature = "date")]
    crate::sql_test!(
        tsrange,
        std::ops::Range<chrono::NaiveDateTime>,
        [(
            "'[1970-01-01 00:00:00, 2010-01-01 00:00:00)'",
            chrono::NaiveDate::from_ymd(1970, 01, 01).and_hms(0, 0, 0)
                ..chrono::NaiveDate::from_ymd(2010, 01, 01).and_hms(0, 0, 0)
        )]
    );

    #[cfg(feature = "date")]
    crate::sql_test!(
        tstzrange,
        std::ops::Range<chrono::DateTime<chrono::Utc>>,
        [(
            "'[1970-01-01 00:00:00+00, 2010-01-01 00:00:00+00)'",
            chrono::DateTime::<chrono::Utc>::from_utc(
                chrono::NaiveDate::from_ymd(1970, 01, 01).and_hms(0, 0, 0),
                chrono::Utc
            )
                ..chrono::DateTime::<chrono::Utc>::from_utc(
                    chrono::NaiveDate::from_ymd(2010, 01, 01).and_hms(0, 0, 0),
                    chrono::Utc
                )
        )]
    );
}
