pub trait FromSql: Sized {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self>;

    fn error(pg_type: &crate::pq::Type, rust_type: &str, raw: Option<&String>) -> crate::Error {
        crate::Error::FromSql {
            pg_type: pg_type.clone(),
            rust_type: rust_type.to_string(),
            value: raw.cloned()
        }
    }
}

impl FromSql for bool {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        match raw.unwrap().as_str() {
            "t" => Ok(true),
            "f" => Ok(false),
            _ => Err(Self::error(ty, "bool", raw)),
        }
    }
}

impl FromSql for f32 {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        match raw.unwrap().parse() {
            Ok(s) => Ok(s),
            _ => Err(Self::error(ty, "f32", raw)),
        }
    }
}

impl FromSql for i32 {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        match raw.unwrap().parse() {
            Ok(s) => Ok(s),
            _ => Err(Self::error(ty, "i32", raw)),
        }
    }
}

impl FromSql for i64 {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        match raw.unwrap().parse() {
            Ok(s) => Ok(s),
            _ => Err(Self::error(ty, "i64", raw)),
        }
    }
}

impl<T: FromSql> FromSql for Option<T> {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        match raw {
            Some(_) => Ok(Some(T::from_sql(ty, raw)?)),
            None => Ok(None),
        }
    }
}

impl FromSql for String {
    fn from_sql(_: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        Ok(raw.unwrap().to_string())
    }
}

impl FromSql for u32 {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        match raw.unwrap().parse() {
            Ok(s) => Ok(s),
            _ => Err(Self::error(ty, "u32", raw)),
        }
    }
}

#[cfg(feature = "date")]
impl FromSql for chrono::DateTime<chrono::offset::FixedOffset> {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        match chrono::DateTime::parse_from_str(raw.unwrap(), "%F %T%#z") {
            Ok(date) => return Ok(date),
            Err(_) => (),
        };

        match chrono::DateTime::parse_from_str(raw.unwrap(), "%F %T.%f%#z") {
            Ok(date) => Ok(date),
            _ => Err(Self::error(ty, "timestamptz", raw)),
        }
    }
}

#[cfg(feature = "serde_json")]
impl FromSql for serde_json::value::Value {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        match serde_json::from_str(&raw.unwrap()) {
            Ok(json) => Ok(json),
            _ => Err(Self::error(ty, "json", raw)),
        }
    }
}

#[cfg(feature = "uuid")]
impl FromSql for uuid::Uuid {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        match uuid::Uuid::parse_str(&raw.unwrap()) {
            Ok(uuid) => Ok(uuid),
            _ => Err(Self::error(ty, "uuid", raw)),
        }
    }
}
