use byteorder::ReadBytesExt;

#[inline]
pub fn not_null<T>(raw: Option<T>) -> crate::Result<T> {
    raw.ok_or(crate::Error::NotNull)
}

macro_rules! read {
    ($fn:ident, $ty:ty) => {
        #[inline]
        #[allow(dead_code)]
        pub fn $fn(buf: &mut &[u8]) -> crate::Result<$ty> {
            let n = buf.$fn::<byteorder::BigEndian>()?;

            Ok(n)
        }
    };
}

read!(read_i16, i16);
read!(read_i32, i32);
read!(read_i64, i64);
read!(read_f32, f32);
read!(read_f64, f64);
read!(read_u32, u32);
read!(read_u128, u128);

#[inline]
#[allow(dead_code)]
pub fn read_i8(buf: &mut &[u8]) -> crate::Result<i8> {
    let n = buf.read_i8()?;

    Ok(n)
}

#[inline]
pub fn read_u8(buf: &mut &[u8]) -> crate::Result<u8> {
    let n = buf.read_u8()?;

    Ok(n)
}

macro_rules! number {
    ($type:ty, $read:ident) => {
        impl FromSql for $type {
            fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
                let mut buf = crate::from_sql::not_null(raw)?;
                let v = $read(&mut buf)?;

                if !buf.is_empty() {
                    return Err(Self::error(ty, raw));
                }

                Ok(v)
            }

            fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
                crate::from_sql::not_null(raw)?
                    .parse()
                    .map_err(|_| Self::error(ty, raw))
            }
        }
    };
}

/**
 * Trait to allow a rust type to be translated form a SQL value.
 */
pub trait FromSql: Sized {
    /**
     * Create a new struct from the binary representation.
     *
     * See the postgresql
     * [adt](https://github.com/postgres/postgres/tree/REL_12_0/src/backend/utils/adt)
     * module source code, mainly `*_recv` functions.
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self>;

    /**
     * Create a new struct from the text representation.
     *
     * See the postgresql
     * [adt](https://github.com/postgres/postgres/tree/REL_12_0/src/backend/utils/adt)
     * module source code, mainly `*_in` functions.
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self>;

    /**
     * Create a new struct from SQL value.
     */
    fn from_sql(
        ty: &crate::pq::Type,
        format: crate::pq::Format,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        match format {
            crate::pq::Format::Binary => Self::from_binary(ty, raw),
            crate::pq::Format::Text => {
                let text = raw.map(|x| String::from_utf8(x.to_vec())).transpose()?;

                Self::from_text(ty, text.as_deref())
            }
        }
    }

    fn error<T: std::fmt::Debug>(pg_type: &crate::pq::Type, raw: T) -> crate::Error {
        crate::Error::FromSql {
            pg_type: pg_type.clone(),
            rust_type: std::any::type_name::<Self>().to_string(),
            value: format!("{raw:?}"),
        }
    }
}

number!(f32, read_f32);
number!(f64, read_f64);
number!(i16, read_i16);
number!(i32, read_i32);
number!(i64, read_i64);

impl FromSql for u16 {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        i32::from_text(ty, raw).map(|x| x as u16)
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let raw = raw.map(|x| {
            let mut vec = vec![0; 4 - x.len()];
            vec.extend_from_slice(x);
            vec
        });

        i32::from_binary(ty, raw.as_deref()).map(|x| x as u16)
    }
}

impl FromSql for u32 {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        i64::from_text(ty, raw).map(|x| x as u32)
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let raw = raw.map(|x| {
            let mut vec = vec![0; 8 - x.len()];
            vec.extend_from_slice(x);
            vec
        });

        i64::from_binary(ty, raw.as_deref()).map(|x| x as u32)
    }
}

impl FromSql for usize {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        not_null(raw)?.parse().map_err(|_| Self::error(ty, raw))
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = not_null(raw)?;
        #[cfg(target_pointer_width = "64")]
        let v = buf.read_u64::<byteorder::BigEndian>()?;
        #[cfg(target_pointer_width = "32")]
        let v = buf.read_u32::<byteorder::BigEndian>()?;

        if !buf.is_empty() {
            return Err(Self::error(ty, raw));
        }

        Ok(v as usize)
    }
}

impl FromSql for bool {
    fn from_text(_: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        Ok(not_null(raw)? == "t")
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let buf = not_null(raw)?;
        if buf.len() != 1 {
            return Err(Self::error(ty, raw));
        }

        Ok(not_null(raw)?[0] != 0)
    }
}

impl<T: FromSql> FromSql for Option<T> {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        match raw {
            Some(_) => Ok(Some(T::from_text(ty, raw)?)),
            None => Ok(None),
        }
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        match raw {
            Some(_) => Ok(Some(T::from_binary(ty, raw)?)),
            None => Ok(None),
        }
    }
}

impl FromSql for char {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        not_null(raw)?
            .chars()
            .next()
            .ok_or_else(|| Self::error(ty, raw))
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let c = String::from_binary(ty, raw)?;

        c.chars().next().ok_or_else(|| Self::error(ty, raw))
    }
}

impl FromSql for String {
    fn from_text(_: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        Ok(not_null(raw)?.to_string())
    }

    fn from_binary(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        String::from_utf8(not_null(raw)?.to_vec()).map_err(Into::into)
    }
}

impl FromSql for () {
    fn from_text(_: &crate::pq::Type, _: Option<&str>) -> crate::Result<Self> {
        Ok(())
    }

    fn from_binary(_: &crate::pq::Type, _: Option<&[u8]>) -> crate::Result<Self> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(float4, f32, [("1.", 1.), ("-1.", -1.), ("2.1", 2.1)]);

    crate::sql_test!(float8, f64, [("1.", 1.), ("-1.", -1.), ("2.1", 2.1)]);

    crate::sql_test!(
        int2,
        i16,
        [
            (i16::MAX.to_string().as_str(), i16::MAX),
            ("1", 1),
            ("0", 0),
            ("-1", -1),
        ]
    );

    crate::sql_test!(
        int,
        u16,
        [
            (u16::MAX.to_string().as_str(), u16::MAX),
            ("1", 1),
            ("0", 0)
        ]
    );

    crate::sql_test!(
        int4,
        i32,
        [
            (i32::MAX.to_string().as_str(), i32::MAX),
            ("1", 1),
            ("0", 0),
            ("-1", -1),
        ]
    );

    crate::sql_test!(
        bigint,
        u32,
        [
            (u32::MAX.to_string().as_str(), u32::MAX),
            ("1", 1),
            ("0", 0)
        ]
    );

    crate::sql_test!(
        int8,
        i64,
        [
            (i64::MAX.to_string().as_str(), i64::MAX),
            ("1", 1),
            ("0", 0),
            ("-1", -1),
        ]
    );

    crate::sql_test!(oid, crate::pq::Oid, [("1", 1)]);

    crate::sql_test!(
        bool,
        bool,
        [
            ("'t'", true),
            ("'f'", false),
            ("true", true),
            ("false", false),
        ]
    );

    crate::sql_test!(char, char, [("'f'", 'f'), ("'('", '(')]);

    crate::sql_test!(varchar, Option<String>, [("null", None::<String>)]);

    crate::sql_test!(
        text,
        String,
        [("'foo'", "foo".to_string()), ("''", "".to_string())]
    );

    crate::sql_test!(us_postal_code, String, [("'12345'", "12345".to_string()),]);

    crate::sql_test!(unknown, (), [("null", ())]);

    #[derive(elephantry_derive::Enum, Debug, PartialEq)]
    enum Mood {
        Sad,
        Ok,
        Happy,
    }

    crate::sql_test!(
        mood,
        super::Mood,
        [
            ("'Sad'", super::Mood::Sad),
            ("'Ok'", super::Mood::Ok),
            ("'Happy'", super::Mood::Happy),
        ]
    );

    #[derive(elephantry_derive::Composite, Debug, PartialEq)]
    struct CompFoo {
        f1: i32,
        f2: String,
    }

    impl crate::entity::Simple for CompFoo {}

    crate::sql_test!(
        compfoo,
        super::CompFoo,
        [(
            "'(1,foo)'",
            super::CompFoo {
                f1: 1,
                f2: "foo".to_string()
            }
        )]
    );
}
