use byteorder::ReadBytesExt;

macro_rules! not_null {
    ($raw:ident) => {
        match $raw {
            Some(raw) => raw,
            None => return Err(crate::Error::NotNull),
        }
    };
}

macro_rules! number {
    ($type:ty, $read:ident) => {
        impl FromSql for $type {
            fn from_binary(
                ty: &crate::pq::Type,
                raw: Option<&[u8]>,
            ) -> crate::Result<Self> {
                let mut buf = not_null!(raw);
                let v = buf.$read::<byteorder::BigEndian>()?;

                if !buf.is_empty() {
                    return Err(Self::error(ty, stringify!($type), raw));
                }

                Ok(v)
            }

            fn from_text(
                ty: &crate::pq::Type,
                raw: Option<&str>,
            ) -> crate::Result<Self> {
                not_null!(raw)
                    .parse()
                    .map_err(|_| Self::error(ty, stringify!($type), raw))
            }
        }
    };
}

pub trait FromSql: Sized {
    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self>;
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self>;

    fn from_sql(
        ty: &crate::pq::Type,
        format: crate::pq::Format,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        match format {
            crate::pq::Format::Binary => Self::from_binary(ty, raw),
            crate::pq::Format::Text => {
                Self::from_text(
                    ty,
                    raw.map(|x| String::from_utf8(x.to_vec()).unwrap())
                        .as_deref(),
                )
            },
        }
    }

    fn error<T: std::fmt::Debug>(
        pg_type: &crate::pq::Type,
        rust_type: &str,
        raw: T,
    ) -> crate::Error {
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

impl FromSql for usize {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        not_null!(raw)
            .parse()
            .map_err(|_| Self::error(ty, "usize", raw))
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let mut buf = not_null!(raw);
        #[cfg(target_pointer_width = "64")]
        let v = buf.read_u64::<byteorder::BigEndian>()?;
        #[cfg(target_pointer_width = "32")]
        let v = buf.read_u32::<byteorder::BigEndian>()?;

        if !buf.is_empty() {
            return Err(Self::error(ty, "usize", raw));
        }

        Ok(v as usize)
    }
}

impl FromSql for bool {
    fn from_text(
        _: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        Ok(not_null!(raw) == "t")
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let buf = not_null!(raw);
        if buf.len() != 1 {
            return Err(Self::error(ty, "bool", raw));
        }

        Ok(not_null!(raw)[0] != 0)
    }
}

impl<T: FromSql> FromSql for Option<T> {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        match raw {
            Some(_) => Ok(Some(T::from_text(ty, raw)?)),
            None => Ok(None),
        }
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        match raw {
            Some(_) => Ok(Some(T::from_binary(ty, raw)?)),
            None => Ok(None),
        }
    }
}

impl FromSql for String {
    fn from_text(
        _: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        Ok(not_null!(raw).to_string())
    }

    fn from_binary(
        _: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        String::from_utf8(not_null!(raw).to_vec()).map_err(|e| e.into())
    }
}

impl<T: FromSql + Clone> FromSql for Vec<T> {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        Ok(crate::Array::from_text(ty, raw)?.into())
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        Ok(crate::Array::from_binary(ty, raw)?.into())
    }
}

#[cfg(feature = "date")]
impl FromSql for chrono::Date<chrono::offset::Utc> {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        let naive = chrono::NaiveDate::from_text(ty, raw)?;
        Ok(chrono::Date::from_utc(naive, chrono::offset::Utc))
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let naive = chrono::NaiveDate::from_binary(ty, raw)?;
        Ok(chrono::Date::from_utc(naive, chrono::offset::Utc))
    }
}

#[cfg(feature = "date")]
impl FromSql for chrono::Date<chrono::offset::FixedOffset> {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        let utc = chrono::Date::<chrono::offset::Utc>::from_text(ty, raw)?;
        Ok(utc.with_timezone(&chrono::offset::FixedOffset::east(0)))
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let utc = chrono::Date::<chrono::offset::Utc>::from_binary(ty, raw)?;
        Ok(utc.with_timezone(&chrono::offset::FixedOffset::east(0)))
    }
}

#[cfg(feature = "date")]
impl FromSql for chrono::Date<chrono::offset::Local> {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        let utc = chrono::Date::<chrono::offset::Utc>::from_text(ty, raw)?;
        Ok(utc.with_timezone(&chrono::offset::Local))
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let utc = chrono::Date::<chrono::offset::Utc>::from_binary(ty, raw)?;
        Ok(utc.with_timezone(&chrono::offset::Local))
    }
}

#[cfg(feature = "date")]
impl FromSql for chrono::NaiveDate {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        match chrono::NaiveDate::parse_from_str(not_null!(raw), "%F") {
            Ok(date) => Ok(date),
            _ => Err(Self::error(ty, "date", raw)),
        }
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let t = i32::from_binary(ty, raw)?;
        let base = chrono::NaiveDate::from_ymd(2000, 1, 1);

        Ok(base + chrono::Duration::days(t.into()))
    }
}

#[cfg(feature = "date")]
impl FromSql for chrono::DateTime<chrono::offset::Utc> {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        let naive = chrono::NaiveDateTime::from_text(ty, raw)?;
        Ok(chrono::DateTime::from_utc(naive, chrono::offset::Utc))
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let naive = chrono::NaiveDateTime::from_binary(ty, raw)?;
        Ok(chrono::DateTime::from_utc(naive, chrono::offset::Utc))
    }
}

#[cfg(feature = "date")]
impl FromSql for chrono::DateTime<chrono::offset::FixedOffset> {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        let utc = chrono::DateTime::<chrono::offset::Utc>::from_text(ty, raw)?;
        Ok(utc.with_timezone(&chrono::offset::FixedOffset::east(0)))
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let utc =
            chrono::DateTime::<chrono::offset::Utc>::from_binary(ty, raw)?;
        Ok(utc.with_timezone(&chrono::offset::FixedOffset::east(0)))
    }
}

#[cfg(feature = "date")]
impl FromSql for chrono::DateTime<chrono::offset::Local> {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        let utc = chrono::DateTime::<chrono::offset::Utc>::from_text(ty, raw)?;
        Ok(utc.with_timezone(&chrono::offset::Local))
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let utc =
            chrono::DateTime::<chrono::offset::Utc>::from_binary(ty, raw)?;
        Ok(utc.with_timezone(&chrono::offset::Local))
    }
}

#[cfg(feature = "date")]
impl FromSql for chrono::NaiveDateTime {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        if let Ok(date) =
            chrono::NaiveDateTime::parse_from_str(not_null!(raw), "%F %T")
        {
            return Ok(date);
        }

        match chrono::NaiveDateTime::parse_from_str(not_null!(raw), "%F %T.%f")
        {
            Ok(date) => Ok(date),
            _ => Err(Self::error(ty, "timestamp", raw)),
        }
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let t = i64::from_binary(ty, raw)?;
        let base = chrono::NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);

        Ok(base + chrono::Duration::microseconds(t))
    }
}

#[cfg(feature = "json")]
impl FromSql for serde_json::value::Value {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        match serde_json::from_str(not_null!(raw)) {
            Ok(json) => Ok(json),
            _ => Err(Self::error(ty, "json", raw)),
        }
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let s = String::from_binary(ty, raw)?;

        match serde_json::from_str(&s) {
            Ok(json) => Ok(json),
            _ => Err(Self::error(ty, "json", raw)),
        }
    }
}

#[cfg(feature = "uuid")]
impl FromSql for uuid::Uuid {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        match uuid::Uuid::parse_str(&not_null!(raw)) {
            Ok(uuid) => Ok(uuid),
            _ => Err(Self::error(ty, "uuid", raw)),
        }
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        todo!();
    }
}

#[cfg(feature = "numeric")]
impl FromSql for bigdecimal::BigDecimal {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        use std::str::FromStr;

        Self::from_str(&not_null!(raw))
            .map_err(|_| Self::error(ty, "numeric", raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/numeric.c#L872
     */
    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        const NBASE: i64 = 10_000;
        const DEC_DIGITS: u32 = 4;

        let mut buf = not_null!(raw);
        let ndigits = buf.read_u16::<byteorder::BigEndian>()? as u32;
        let weight = buf.read_u16::<byteorder::BigEndian>()? as u32;
        let sign = buf.read_u16::<byteorder::BigEndian>()?;
        let dscale = buf.read_u16::<byteorder::BigEndian>()?;

        let mut result = bigdecimal::BigDecimal::default();

        if ndigits == 0 {
            return Ok(result);
        }

        result = match sign {
            0 => result,
            0x4000 => -result,
            0xC000 => return Err(Self::error(ty, "numeric", raw)),
            _ => return Err(Self::error(ty, "numeric", raw)),
        };

        let first_digit = buf.read_i16::<byteorder::BigEndian>()?;
        result += bigdecimal::BigDecimal::from(
            first_digit as i64 * NBASE.pow(weight),
        );

        for _ in 1..weight {
            let digit = buf.read_i16::<byteorder::BigEndian>()?;

            result *= bigdecimal::BigDecimal::from(NBASE);
            result += bigdecimal::BigDecimal::from(digit);
        }

        if dscale > 0 {
            for x in weight + 1..ndigits {
                let digit = buf.read_i16::<byteorder::BigEndian>()?;
                result += bigdecimal::BigDecimal::from(
                    digit as f32 / (10_u32.pow(DEC_DIGITS) * x) as f32,
                );
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    macro_rules! from_test {
        ($sql_type:ident, $rust_type:ty, $tests:expr) => {
            #[test]
            fn $sql_type() -> crate::Result<()> {
                let conn = crate::test::new_conn();
                for (value, expected) in &$tests {
                    // from text
                    let result = conn.execute(&format!("select {}::{} as actual", value, stringify!($sql_type)))?;
                    assert_eq!(result.get(0).get::<$rust_type>("actual"), *expected);

                    // from binary
                    let result = conn.query::<HashMap<String, $rust_type>>(&format!("select {}::{} as actual", value, stringify!($sql_type)), &[])?;
                    assert_eq!(result.get(0).get("actual").unwrap(), expected);
                }

                Ok(())
            }

        }
    }

    from_test!(float4, f32, [
        (1., 1.),
        (-1., -1.),
        (2.1, 2.1),
    ]);

    from_test!(int4, i32, [
        (i32::MAX, i32::MAX),
        (1, 1),
        (0, 0),
        (-1, -1),
    ]);

    from_test!(int8, i64, [
        (i64::MAX, i64::MAX),
        (1, 1),
        (0, 0),
        (-1, -1),
    ]);

    from_test!(bool, bool, [
        ("'t'", true),
        ("'f'", false),
        ("true", true),
        ("false", false),
    ]);

    from_test!(varchar, Option<String>, [
        ("null", None),
    ]);

    from_test!(text, String, [
        ("'foo'", "foo"),
        ("''", ""),
    ]);

    #[cfg(feature = "date")]
    from_test!(date, chrono::NaiveDate, [
        ("'1970-01-01'", chrono::NaiveDate::from_ymd(1970, 01, 01)),
        ("'2010-01-01'", chrono::NaiveDate::from_ymd(2010, 01, 01)),
        ("'2100-12-30'", chrono::NaiveDate::from_ymd(2100, 12, 30)),
    ]);

    #[cfg(feature = "date")]
    from_test!(timestamp, chrono::NaiveDateTime, [
        ("'1970-01-01 00:00:00'", chrono::NaiveDateTime::from_timestamp(0, 0)),
    ]);

    #[cfg(feature = "json")]
    from_test!(json, serde_json::value::Value, [
        ("'{\"foo\": \"bar\"}'", serde_json::json!({"foo": "bar"})),
    ]);

    #[cfg(feature = "uuid")]
    from_test!(uuid, uuid::Uuid, [
        ("'12edd47f-e2fc-44eb-9419-1995dfb6725d'", uuid::Uuid::parse_str("12edd47f-e2fc-44eb-9419-1995dfb6725d").unwrap()),
    ]);

    #[cfg(feature = "numeric")]
    from_test!(numeric, bigdecimal::BigDecimal, [
        ("20000", bigdecimal::BigDecimal::from(20_000.)),
        ("3900", bigdecimal::BigDecimal::from(3_900.)),
        ("3900.98", bigdecimal::BigDecimal::from(3_900.98)),
    ]);
}
