mod interval;
mod timestamp;
mod timestamptz;

pub use interval::*;

#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
impl crate::ToSql for chrono::NaiveDate {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::DATE
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L179
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F").to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L226
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        use byteorder::WriteBytesExt;

        let base = chrono::NaiveDate::from_ymd(2000, 1, 1);
        let t: chrono::Duration = *self - base;

        let mut buf = Vec::new();
        buf.write_i32::<byteorder::BigEndian>(t.num_days() as i32)?;

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
impl crate::FromSql for chrono::NaiveDate {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L114
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        match chrono::NaiveDate::parse_from_str(crate::not_null(raw)?, "%F") {
            Ok(date) => Ok(date),
            _ => Err(Self::error(ty, "date", raw)),
        }
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L204
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let t = i32::from_binary(ty, raw)?;
        let base = chrono::NaiveDate::from_ymd(2000, 1, 1);

        Ok(base + chrono::Duration::days(t.into()))
    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(
        date,
        chrono::NaiveDate,
        [
            ("'1970-01-01'", chrono::NaiveDate::from_ymd(1970, 01, 01)),
            ("'2010-01-01'", chrono::NaiveDate::from_ymd(2010, 01, 01)),
            ("'2100-12-30'", chrono::NaiveDate::from_ymd(2100, 12, 30)),
        ]
    );
}
