pub trait FromText: Sized {
    fn from_text(raw: &str) -> crate::Result<Self>;

    fn error(raw: &str) -> crate::Error {
        crate::Error::FromSql {
            pg_type: crate::pq::types::TEXT,
            rust_type: std::any::type_name::<Self>().to_string(),
            value: raw.to_string(),
        }
    }
}

impl<T: FromText> crate::FromSql for T {
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        String::from_binary(ty, raw).and_then(|x| T::from_text(&x))
    }

    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        String::from_text(ty, raw).and_then(|x| T::from_text(&x))
    }
}
