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

#[cfg(feature = "date")]
impl ToSql for chrono::Date<chrono::offset::FixedOffset> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMPTZ
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F").to_string().to_sql()
    }
}

#[cfg(feature = "date")]
impl ToSql for chrono::Date<chrono::offset::Utc> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMPTZ
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F").to_string().to_sql()
    }
}

#[cfg(feature = "date")]
impl ToSql for chrono::Date<chrono::offset::Local> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMPTZ
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F").to_string().to_sql()
    }
}

#[cfg(feature = "date")]
impl ToSql for chrono::NaiveDate {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMP
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F").to_string().to_sql()
    }
}

#[cfg(feature = "date")]
impl ToSql for chrono::DateTime<chrono::offset::FixedOffset> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMPTZ
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_rfc2822().to_sql()
    }
}

#[cfg(feature = "date")]
impl ToSql for chrono::DateTime<chrono::offset::Utc> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMPTZ
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_rfc2822().to_sql()
    }
}

#[cfg(feature = "date")]
impl ToSql for chrono::DateTime<chrono::offset::Local> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMPTZ
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F %T").to_string().to_sql()
    }
}

#[cfg(feature = "date")]
impl ToSql for chrono::NaiveDateTime {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMP
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F %T").to_string().to_sql()
    }
}

#[cfg(feature = "json")]
impl ToSql for serde_json::value::Value {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::JSON
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        match serde_json::to_string(self) {
            Ok(s) => s.to_sql(),
            Err(err) => Err(self.error("json", Some(&err.to_string()))),
        }
    }
}

#[cfg(feature = "uuid")]
impl ToSql for uuid::Uuid {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::UUID
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

#[cfg(feature = "numeric")]
impl ToSql for bigdecimal::BigDecimal {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::NUMERIC
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    macro_rules! to_test {
        ($sql_type:ident, $tests:expr) => {
            #[test]
            fn $sql_type() -> crate::Result<()> {
                let conn = crate::test::new_conn();
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

    #[cfg(feature = "date")]
    to_test!(date, [
        chrono::NaiveDate::from_ymd(1970, 01, 01),
        chrono::NaiveDate::from_ymd(2010, 01, 01),
        chrono::NaiveDate::from_ymd(2100, 12, 30),
    ]);

    #[cfg(feature = "date")]
    to_test!(timestamp, [
        chrono::NaiveDateTime::from_timestamp(0, 0),
    ]);

    #[cfg(feature = "json")]
    to_test!(json, [
        serde_json::json!({"foo": "bar"}),
    ]);

    #[cfg(feature = "uuid")]
    to_test!(uuid, [
        uuid::Uuid::parse_str("12edd47f-e2fc-44eb-9419-1995dfb6725d").unwrap(),
    ]);

    #[cfg(feature = "numeric")]
    to_test!(numeric, [
        bigdecimal::BigDecimal::from(20_000.),
        bigdecimal::BigDecimal::from(3_900.),
        bigdecimal::BigDecimal::from(3_900.98),
    ]);

    #[test]
    fn vec_to_sql() {
        use crate::ToSql;

        let vec = vec![1, 2, 3];

        assert_eq!(vec.to_sql().unwrap(), Some(b"{1,2,3}\0".to_vec()));
    }
}
