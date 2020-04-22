pub trait FromSql: Sized {
    fn from_sql(ty: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self>;
}

impl FromSql for bool {
    fn from_sql(_: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        match raw.unwrap().as_str() {
            "t" => Ok(true),
            "f" => Ok(false),
            _ => Err(format!("Invalid bool value: '{:?}'", raw)),
        }
    }
}

impl FromSql for i32 {
    fn from_sql(_: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        match raw.unwrap().parse() {
            Ok(s) => Ok(s),
            Err(_) => Err(format!("Invalid i32 value: '{:?}'", raw)),
        }
    }
}

impl FromSql for i64 {
    fn from_sql(_: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        match raw.unwrap().parse() {
            Ok(s) => Ok(s),
            Err(_) => Err(format!("Invalid i64 value: '{:?}'", raw)),
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
    fn from_sql(_: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        match raw.unwrap().parse() {
            Ok(s) => Ok(s),
            Err(_) => Err(format!("Invalid i32 value: {:?}", raw.unwrap())),
        }
    }
}

#[cfg(feature = "serde_json")]
impl FromSql for serde_json::value::Value {
    fn from_sql(_: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        match serde_json::from_str(&raw.unwrap()) {
            Ok(json) => Ok(json),
            Err(_) => Err(format!("Invalid json value: '{:?}'", raw)),
        }
    }
}

#[cfg(feature = "uuid")]
impl FromSql for uuid::Uuid {
    fn from_sql(_: &crate::pq::Type, raw: Option<&String>) -> crate::Result<Self> {
        match uuid::Uuid::parse_str(&raw.unwrap()) {
            Ok(uuid) => Ok(uuid),
            Err(_) => Err(format!("Invalid uuid value: '{:?}'", raw)),
        }
    }
}
