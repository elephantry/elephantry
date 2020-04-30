use byteorder::ReadBytesExt;

macro_rules! number {
    ($type:ty, $read:ident) => {
        impl FromSql for $type {
            fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
                let mut buf = raw.unwrap();
                let v = buf.$read::<byteorder::BigEndian>()?;

                if !buf.is_empty() {
                    return Err(Self::error(ty, stringify!($type), raw));
                }

                Ok(v)
            }

            fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
                raw.unwrap().parse()
                    .map_err(|_| Self::error(ty, stringify!($type), raw))
            }
        }
    }
}

pub trait FromSql: Sized {
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self>;
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self>;

    fn from_sql(ty: &crate::pq::Type, format: crate::pq::Format, raw: Option<&[u8]>) -> crate::Result<Self> {
        match format {
            crate::pq::Format::Binary => Self::from_binary(ty, raw),
            crate::pq::Format::Text => Self::from_text(
                ty,
                raw.map(|x| String::from_utf8(x.to_vec()).unwrap()).as_deref()
            ),
        }
    }

    fn error<T: std::fmt::Debug>(pg_type: &crate::pq::Type, rust_type: &str, raw: T) -> crate::Error {
        crate::Error::FromSql {
            pg_type: pg_type.clone(),
            rust_type: rust_type.to_string(),
            value: format!("{:?}", raw),
        }
    }
}

number!(f32, read_f32);
number!(i32, read_i32);
number!(i64, read_i64);
number!(u32, read_u32);

impl FromSql for bool {
    fn from_text(_: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        Ok(raw.unwrap() == "t")
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let buf = raw.unwrap();
        if buf.len() != 1 {
            return Err(Self::error(ty, "bool", raw));
        }

        Ok(raw.unwrap()[0] != 0)
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

impl FromSql for String {
    fn from_text(_: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        Ok(raw.unwrap().to_string())
    }

    fn from_binary(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        String::from_utf8(raw.unwrap().to_vec()).map_err(|e| e.into()) }
}

#[cfg(feature = "date")]
impl FromSql for chrono::DateTime<chrono::offset::Utc> {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let naive = chrono::NaiveDateTime::from_text(ty, raw)?;
        Ok(chrono::DateTime::from_utc(naive, chrono::offset::Utc))
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let naive = chrono::NaiveDateTime::from_binary(ty, raw)?;
        Ok(chrono::DateTime::from_utc(naive, chrono::offset::Utc))
    }
}

#[cfg(feature = "date")]
impl FromSql for chrono::DateTime<chrono::offset::FixedOffset> {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let utc = chrono::DateTime::<chrono::offset::Utc>::from_text(ty, raw)?;
        Ok(utc.with_timezone(&chrono::offset::FixedOffset::east(0)))
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let utc = chrono::DateTime::<chrono::offset::Utc>::from_binary(ty, raw)?;
        Ok(utc.with_timezone(&chrono::offset::FixedOffset::east(0)))
    }
}

#[cfg(feature = "date")]
impl FromSql for chrono::NaiveDateTime {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        if let Ok(date) = chrono::NaiveDateTime::parse_from_str(raw.unwrap(), "%F %T") {
            return Ok(date);
        }

        match chrono::NaiveDateTime::parse_from_str(raw.unwrap(), "%F %T.%f") {
            Ok(date) => Ok(date),
            _ => Err(Self::error(ty, "timestamp", raw)),
        }
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let t = i64::from_binary(ty, raw)?;
        let base = chrono::NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);

        Ok(base + chrono::Duration::microseconds(t))
    }
}

#[cfg(feature = "json")]
impl FromSql for serde_json::value::Value {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        match serde_json::from_str(raw.unwrap()) {
            Ok(json) => Ok(json),
            _ => Err(Self::error(ty, "json", raw)),
        }
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let s = String::from_binary(ty, raw)?;

        match serde_json::from_str(&s) {
            Ok(json) => Ok(json),
            _ => Err(Self::error(ty, "json", raw)),
        }
    }
}

#[cfg(feature = "uuid")]
impl FromSql for uuid::Uuid {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        match uuid::Uuid::parse_str(&raw.unwrap()) {
            Ok(uuid) => Ok(uuid),
            _ => Err(Self::error(ty, "uuid", raw)),
        }
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let s = String::from_binary(ty, raw)?;

        match uuid::Uuid::parse_str(&s) {
            Ok(uuid) => Ok(uuid),
            _ => Err(Self::error(ty, "uuid", raw)),
        }
    }
}
