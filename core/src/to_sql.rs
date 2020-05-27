pub trait ToSql {
    fn ty(&self) -> crate::pq::Type;
    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>>;

    fn format(&self) -> crate::pq::Format {
        crate::pq::Format::Text
    }

    fn error(&self, rust_type: &str, message: Option<&String>) -> crate::Error {
        crate::Error::ToSql {
            pg_type: self.ty(),
            rust_type: rust_type.to_string(),
            message: message.cloned(),
        }
    }
}

impl ToSql for bool {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::BOOL
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        let v = if *self { b"t\0" } else { b"f\0" };

        Ok(Some(v.to_vec()))
    }
}

impl ToSql for f32 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::FLOAT4
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

impl ToSql for f64 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::FLOAT8
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

impl ToSql for &str {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::VARCHAR
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut v = self.as_bytes().to_vec();
        v.push(0);

        Ok(Some(v))
    }
}

impl ToSql for char {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::BPCHAR
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

impl ToSql for String {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::VARCHAR
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.as_str().to_sql()
    }
}

impl ToSql for i16 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::INT2
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

impl ToSql for i32 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::INT4
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

impl ToSql for i64 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::INT8
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

impl ToSql for u32 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::INT8
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

impl<T: ToSql> ToSql for Option<T> {
    fn ty(&self) -> crate::pq::Type {
        match self {
            Some(data) => data.ty(),
            None => crate::pq::ty::TEXT,
        }
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        match self {
            Some(data) => T::to_sql(data),
            None => Ok(None),
        }
    }
}

impl<T: ToSql> ToSql for Vec<T> {
    fn ty(&self) -> crate::pq::Type {
        match self.get(0) {
            Some(data) => data.ty(),
            None => crate::pq::ty::TEXT,
        }
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut data = Vec::new();

        data.push(b'{');
        for x in self {
            let element = match x.to_sql()? {
                Some(element) => element,
                None => b"null\0".to_vec(),
            };

            data.extend_from_slice(&element[..element.len() - 1]);
            data.push(b',');
        }
        *data.last_mut().unwrap() = b'}';
        data.push(b'\0');

        Ok(Some(data))
    }
}

#[cfg(test)]
mod test {
    #[macro_export]
    macro_rules! to_test {
        ($sql_type:ident, $tests:expr) => {
            #[test]
            fn $sql_type() -> crate::Result<()> {
                use std::collections::HashMap;

                let conn = crate::test::new_conn();
                conn.execute("set lc_monetary to 'en_US.UTF-8';")?;

                for value in &$tests {
                    let result = conn.query::<HashMap<String, String>>(
                        &format!("select $1::{}", stringify!($sql_type)),
                        &[value],
                    );
                    assert!(result.is_ok());
                }

                Ok(())
            }
        }
    }

    to_test!(float4, [1., -1., 2.1]);
    to_test!(float8, [1., -1., 2.1]);
    to_test!(int2, [i16::MAX, 1, 0, -1]);
    to_test!(int4, [i32::MAX, 1, 0, -1]);
    to_test!(int8, [i64::MAX, 1, 0, -1]);
    to_test!(bool, [true, false]);
    to_test!(char, ['f', 'à']);
    to_test!(varchar, [None::<String>]);
    to_test!(text, ["foo", ""]);

    #[test]
    fn vec_to_sql() {
        use crate::ToSql;

        let vec = vec![1, 2, 3];

        assert_eq!(vec.to_sql().unwrap(), Some(b"{1,2,3}\0".to_vec()));
    }
}
