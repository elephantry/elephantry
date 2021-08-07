/**
 * Trait to allow a rust type to be translated to a SQL value.
 */
pub trait ToSql {
    /** The corresponding SQL type */
    fn ty(&self) -> crate::pq::Type;
    /** Convert the value to text format */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>>;

    /** Convert the value to the prefered format specified by `ToSql::format()` */
    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        if self.format() == crate::pq::Format::Text {
            self.to_text()
        } else {
            self.to_binary()
        }
    }

    /** Convert the value to binary format */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        unimplemented!()
    }

    /** Prefered default format */
    fn format(&self) -> crate::pq::Format {
        crate::pq::Format::Text
    }

    fn error(&self, _rust_type: &str, message: Option<&String>) -> crate::Error {
        crate::Error::ToSql {
            pg_type: self.ty(),
            rust_type: std::any::type_name::<Self>().to_string(),
            message: message.cloned(),
        }
    }
}

impl ToSql for bool {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::BOOL
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        let v = if *self { b"t\0" } else { b"f\0" };

        Ok(Some(v.to_vec()))
    }
}

impl ToSql for f32 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::FLOAT4
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }
}

impl ToSql for f64 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::FLOAT8
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }
}

impl ToSql for &str {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::VARCHAR
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut v = self.as_bytes().to_vec();
        v.push(0);

        Ok(Some(v))
    }
}

impl ToSql for char {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::BPCHAR
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }
}

impl ToSql for String {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::VARCHAR
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.as_str().to_text()
    }
}

impl ToSql for i16 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::INT2
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }
}

impl ToSql for i32 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::INT4
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }
}

impl ToSql for i64 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::INT8
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }
}

impl ToSql for u32 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::INT8
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }
}

impl<T: ToSql> ToSql for Option<T> {
    fn ty(&self) -> crate::pq::Type {
        match self {
            Some(data) => data.ty(),
            None => crate::pq::types::UNKNOWN,
        }
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        match self {
            Some(data) => T::to_text(data),
            None => Ok(None),
        }
    }
}

impl ToSql for () {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::UNKNOWN
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        Ok(None)
    }
}
