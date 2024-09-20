mod interval;
mod timestamp;
mod timestamptz;

pub use interval::*;
pub use timestamp::*;
pub use timestamptz::*;

fn base_date() -> crate::Result<Date> {
    base_datetime().map(|x| x.date())
}

fn base_datetime() -> crate::Result<Timestamp> {
    chrono::NaiveDate::from_ymd_opt(2000, 1, 1)
        .and_then(|x| x.and_hms_opt(0, 0, 0))
        .ok_or_else(|| crate::Error::Chrono("Invalid base date".to_string()))
}

/**
 * Rust type for [date](https://www.postgresql.org/docs/current/datatype-datetime.html).
 */
#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
pub type Date = chrono::NaiveDate;

#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
impl crate::ToSql for Date {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::DATE
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L179
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.format("%F").to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L226
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let t: chrono::Duration = *self - base_date()?;

        let mut buf = Vec::new();
        crate::to_sql::write_i32(&mut buf, t.num_days() as i32)?;

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
impl crate::FromSql for Date {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L114
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        match Date::parse_from_str(crate::from_sql::not_null(raw)?, "%F") {
            Ok(date) => Ok(date),
            _ => Err(Self::error(ty, raw)),
        }
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L204
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let t = i32::from_binary(ty, raw)?;

        Ok(base_date()? + chrono::Duration::days(t.into()))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
impl crate::entity::Simple for Date {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        date,
        crate::Date,
        [
            (
                "'1970-01-01'",
                crate::Date::from_ymd_opt(1970, 1, 1).unwrap()
            ),
            (
                "'2010-01-01'",
                crate::Date::from_ymd_opt(2010, 1, 1).unwrap()
            ),
            (
                "'2100-12-30'",
                crate::Date::from_ymd_opt(2100, 12, 30).unwrap()
            ),
        ]
    );
}
