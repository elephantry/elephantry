use byteorder::ReadBytesExt;

macro_rules! number {
    ($type:ty, $read:ident) => {
        impl FromSql for $type {
            fn from_sql(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
                let mut buf = raw.unwrap();
                let v = buf.$read::<byteorder::BigEndian>()?;

                if !buf.is_empty() {
                    return Err(Self::error(ty, stringify!($type), raw));
                }

                Ok(v)
            }
        }
    }
}

pub trait FromSql: Sized {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self>;

    fn error(pg_type: &crate::pq::Type, rust_type: &str, raw: Option<&[u8]>) -> crate::Error {
        crate::Error::FromSql {
            pg_type: pg_type.clone(),
            rust_type: rust_type.to_string(),
            value: raw.map(|x| x.to_vec())
        }
    }
}

number!(f32, read_f32);
number!(i32, read_i32);
number!(i64, read_i64);
number!(u32, read_u32);

impl FromSql for bool {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let buf = raw.unwrap();
        if buf.len() != 1 {
            return Err(Self::error(ty, "bool", raw));
        }

        Ok(raw.unwrap()[0] != 0)
    }
}

impl<T: FromSql> FromSql for Option<T> {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        match raw {
            Some(_) => Ok(Some(T::from_sql(ty, raw)?)),
            None => Ok(None),
        }
    }
}

impl FromSql for String {
    fn from_sql(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        String::from_utf8(raw.unwrap().to_vec()).map_err(|e| e.into())
    }
}

#[cfg(feature = "date")]
impl FromSql for chrono::DateTime<chrono::offset::Utc> {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let naive = chrono::NaiveDateTime::from_sql(ty, raw)?;
        Ok(chrono::DateTime::from_utc(naive, chrono::offset::Utc))
    }
}

#[cfg(feature = "date")]
impl FromSql for chrono::DateTime<chrono::offset::FixedOffset> {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let utc = chrono::DateTime::<chrono::offset::Utc>::from_sql(ty, raw)?;
        Ok(utc.with_timezone(&chrono::offset::FixedOffset::east(0)))
    }
}

#[cfg(feature = "date")]
impl FromSql for chrono::NaiveDateTime {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let t = i64::from_sql(ty, raw)?;
        let base = chrono::NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);

        Ok(base + chrono::Duration::microseconds(t))
    }
}

#[cfg(feature = "json")]
impl FromSql for serde_json::value::Value {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let s = String::from_sql(ty, raw)?;

        match serde_json::from_str(&s) {
            Ok(json) => Ok(json),
            _ => Err(Self::error(ty, "json", raw)),
        }
    }
}

#[cfg(feature = "uuid")]
impl FromSql for uuid::Uuid {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let s = String::from_sql(ty, raw)?;

        match uuid::Uuid::parse_str(&s) {
            Ok(uuid) => Ok(uuid),
            _ => Err(Self::error(ty, "uuid", raw)),
        }
    }
}
