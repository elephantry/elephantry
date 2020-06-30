use byteorder::ReadBytesExt;

#[inline]
pub(crate) fn not_null<T>(raw: Option<T>) -> crate::Result<T> {
    raw.ok_or(crate::Error::NotNull)
}

macro_rules! number {
    ($type:ty, $read:ident) => {
        impl FromSql for $type {
            fn from_binary(
                ty: &crate::pq::Type,
                raw: Option<&[u8]>,
            ) -> crate::Result<Self> {
                let mut buf = crate::not_null(raw)?;
                let v = buf.$read::<byteorder::BigEndian>()?;

                if !buf.is_empty() {
                    return Err(Self::error(ty, stringify!($type), raw));
                }

                Ok(v)
            }

            fn from_text(
                ty: &crate::pq::Type,
                raw: Option<&str>,
            ) -> crate::Result<Self> {
                crate::not_null(raw)?
                    .parse()
                    .map_err(|_| Self::error(ty, stringify!($type), raw))
            }
        }
    };
}

pub trait FromSql: Sized {
    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self>;
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self>;

    fn from_sql(
        ty: &crate::pq::Type,
        format: crate::pq::Format,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        match format {
            crate::pq::Format::Binary => Self::from_binary(ty, raw),
            crate::pq::Format::Text => {
                Self::from_text(
                    ty,
                    raw.map(|x| String::from_utf8(x.to_vec()).unwrap())
                        .as_deref(),
                )
            },
        }
    }

    fn error<T: std::fmt::Debug>(
        pg_type: &crate::pq::Type,
        rust_type: &str,
        raw: T,
    ) -> crate::Error {
        crate::Error::FromSql {
            pg_type: pg_type.clone(),
            rust_type: rust_type.to_string(),
            value: format!("{:?}", raw),
        }
    }
}

number!(f32, read_f32);
number!(f64, read_f64);
number!(i16, read_i16);
number!(i32, read_i32);
number!(i64, read_i64);
number!(u32, read_u32);

impl FromSql for usize {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        not_null(raw)?
            .parse()
            .map_err(|_| Self::error(ty, "usize", raw))
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let mut buf = not_null(raw)?;
        #[cfg(target_pointer_width = "64")]
        let v = buf.read_u64::<byteorder::BigEndian>()?;
        #[cfg(target_pointer_width = "32")]
        let v = buf.read_u32::<byteorder::BigEndian>()?;

        if !buf.is_empty() {
            return Err(Self::error(ty, "usize", raw));
        }

        Ok(v as usize)
    }
}

impl FromSql for bool {
    fn from_text(
        _: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        Ok(not_null(raw)? == "t")
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let buf = not_null(raw)?;
        if buf.len() != 1 {
            return Err(Self::error(ty, "bool", raw));
        }

        Ok(not_null(raw)?[0] != 0)
    }
}

impl<T: FromSql> FromSql for Option<T> {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        match raw {
            Some(_) => Ok(Some(T::from_text(ty, raw)?)),
            None => Ok(None),
        }
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        match raw {
            Some(_) => Ok(Some(T::from_binary(ty, raw)?)),
            None => Ok(None),
        }
    }
}

impl FromSql for char {
    fn from_text(
        _: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        Ok(not_null(raw)?.chars().next().unwrap())
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let c = String::from_binary(ty, raw)?;

        c.chars().next().ok_or_else(|| Self::error(ty, "char", raw))
    }
}

impl FromSql for String {
    fn from_text(
        _: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        Ok(not_null(raw)?.to_string())
    }

    fn from_binary(
        _: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        String::from_utf8(not_null(raw)?.to_vec()).map_err(|e| e.into())
    }
}

impl<T: FromSql> FromSql for Vec<T> {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        Ok(crate::Array::from_text(ty, raw)?.into())
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        Ok(crate::Array::from_binary(ty, raw)?.into())
    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(float4, f32, [(1., 1.), (-1., -1.), (2.1, 2.1)]);

    crate::sql_test!(float8, f64, [(1., 1.), (-1., -1.), (2.1, 2.1)]);

    crate::sql_test!(int2, i16, [
        (i16::MAX, i16::MAX),
        (1, 1),
        (0, 0),
        (-1, -1),
    ]);

    crate::sql_test!(int4, i32, [
        (i32::MAX, i32::MAX),
        (1, 1),
        (0, 0),
        (-1, -1),
    ]);

    crate::sql_test!(int8, i64, [
        (i64::MAX, i64::MAX),
        (1, 1),
        (0, 0),
        (-1, -1),
    ]);

    crate::sql_test!(bool, bool, [
        ("'t'", true),
        ("'f'", false),
        ("true", true),
        ("false", false),
    ]);

    crate::sql_test!(char, char, [("'f'", 'f'), ("'à'", 'à')]);

    crate::sql_test!(varchar, Option<String>, [("null", None::<String>)]);

    crate::sql_test!(text, String, [("'foo'", "foo"), ("''", "")]);

    crate::sql_test!(us_postal_code, String, [
        ("'12345'", "12345".to_string()),
    ]);
}
