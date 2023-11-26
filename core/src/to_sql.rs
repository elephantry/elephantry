use byteorder::WriteBytesExt;

macro_rules! write {
    ($fn:ident, $ty: ty) => {
        #[inline]
        pub(crate) fn $fn(buf: &mut Vec<u8>, data: $ty) -> crate::Result<()> {
            buf.$fn::<byteorder::BigEndian>(data)?;

            Ok(())
        }
    };
}

write!(write_i16, i16);
write!(write_i32, i32);
write!(write_i64, i64);
write!(write_f32, f32);
write!(write_f64, f64);

#[inline]
#[allow(dead_code)]
pub(crate) fn write_i8(buf: &mut Vec<u8>, data: i8) -> crate::Result<()> {
    buf.write_i8(data)?;

    Ok(())
}

/**
 * Trait to allow a rust type to be translated to a SQL value.
 */
pub trait ToSql {
    /** The corresponding SQL type */
    fn ty(&self) -> crate::pq::Type;

    /**
     * Convert the value to text format
     *
     * See the postgresql
     * [adt](https://github.com/postgres/postgres/tree/REL_12_0/src/backend/utils/adt)
     * module source code, mainly `*_out` functions.
     */
    fn to_text(&self) -> crate::Result<Option<String>>;

    /**
     * Convert the value to binary format
     *
     * See the postgresql
     * [adt](https://github.com/postgres/postgres/tree/REL_12_0/src/backend/utils/adt)
     * module source code, mainly `*_send` functions.
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>>;

    fn error(&self, message: &str) -> crate::Error {
        crate::Error::ToSql {
            pg_type: self.ty(),
            rust_type: std::any::type_name::<Self>().to_string(),
            message: message.to_string(),
        }
    }
}

macro_rules! number {
    ($sql_type:ident, $rust_type:ty, $write:ident) => {
        impl ToSql for $rust_type {
            fn ty(&self) -> crate::pq::Type {
                crate::pq::types::$sql_type
            }

            fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
                let mut buf = Vec::new();
                $write(&mut buf, *self)?;

                Ok(Some(buf))
            }

            fn to_text(&self) -> crate::Result<Option<String>> {
                self.to_string().to_text()
            }
        }
    };
}

number!(FLOAT4, f32, write_f32);
number!(FLOAT8, f64, write_f64);
number!(INT2, i16, write_i16);
number!(INT4, i32, write_i32);
number!(INT8, i64, write_i64);

impl ToSql for u16 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::INT4
    }

    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        i32::from(*self).to_binary()
    }

    fn to_text(&self) -> crate::Result<Option<String>> {
        i32::from(*self).to_text()
    }
}

impl ToSql for u32 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::INT8
    }

    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        i64::from(*self).to_binary()
    }

    fn to_text(&self) -> crate::Result<Option<String>> {
        i64::from(*self).to_text()
    }
}

impl ToSql for bool {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::BOOL
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/bool.c#L164
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        let data = if *self { "t" } else { "f" };

        data.to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/bool.c#L181
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        Ok(Some(vec![u8::from(*self)]))
    }
}

impl ToSql for &str {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::TEXT
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varchar.c#L489
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        (*self).to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varchar.c#L522
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        (*self).to_string().to_binary()
    }
}

impl ToSql for char {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::CHAR
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/char.c#L33
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/char.c#L66
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        Ok(Some(vec![*self as u8]))
    }
}

impl ToSql for String {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::TEXT
    }

    fn to_text(&self) -> crate::Result<Option<String>> {
        Ok(Some(self.clone()))
    }

    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        Ok(Some(self.clone().into_bytes()))
    }
}

impl<T: ToSql> ToSql for Option<T> {
    fn ty(&self) -> crate::pq::Type {
        match self {
            Some(data) => data.ty(),
            None => crate::pq::types::UNKNOWN,
        }
    }

    fn to_text(&self) -> crate::Result<Option<String>> {
        match self {
            Some(data) => T::to_text(data),
            None => Ok(None),
        }
    }

    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        match self {
            Some(data) => T::to_binary(data),
            None => Ok(None),
        }
    }
}

impl ToSql for () {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::UNKNOWN
    }

    fn to_text(&self) -> crate::Result<Option<String>> {
        Ok(None)
    }

    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        Ok(None)
    }
}
