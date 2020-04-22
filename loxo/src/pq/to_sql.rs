pub trait ToSql {
    fn ty(&self) -> crate::pq::Type;
    fn to_sql(&self) -> crate::Result<Vec<u8>>;

    fn format(&self) -> crate::pq::Format {
        crate::pq::Format::Text
    }
}

impl ToSql for bool {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::BOOL
    }

    fn to_sql(&self) -> crate::Result<Vec<u8>> {
        let v = if *self { b"t\0" } else { b"f\0" };

        Ok(v.to_vec())
    }
}

impl ToSql for f32 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::FLOAT4
    }

    fn to_sql(&self) -> crate::Result<Vec<u8>> {
        self.to_string().to_sql()
    }
}

impl ToSql for &str {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::VARCHAR
    }

    fn to_sql(&self) -> crate::Result<Vec<u8>> {
        let mut v = self.as_bytes().to_vec();
        v.push(0);

        Ok(v)
    }
}

impl ToSql for String {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::VARCHAR
    }

    fn to_sql(&self) -> crate::Result<Vec<u8>> {
        self.as_str().to_sql()
    }
}

impl ToSql for i32 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::INT4
    }

    fn to_sql(&self) -> crate::Result<Vec<u8>> {
        self.to_string().to_sql()
    }
}

impl ToSql for u32 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::INT8
    }

    fn to_sql(&self) -> crate::Result<Vec<u8>> {
        self.to_string().to_sql()
    }
}

#[cfg(feature = "date")]
impl ToSql for chrono::DateTime<chrono::offset::FixedOffset> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMPTZ
    }

    fn to_sql(&self) -> crate::Result<Vec<u8>> {
        self.to_rfc2822().to_sql()
    }
}

#[cfg(feature = "json")]
impl ToSql for serde_json::value::Value {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::JSON
    }

    fn to_sql(&self) -> crate::Result<Vec<u8>> {
        match serde_json::to_string(self) {
            Ok(s) => s.to_sql(),
            Err(err) => Err(format!("{}", err)),
        }
    }
}

#[cfg(feature = "uuid")]
impl ToSql for uuid::Uuid {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::UUID
    }

    fn to_sql(&self) -> crate::Result<Vec<u8>> {
        self.to_string().to_sql()
    }
}
