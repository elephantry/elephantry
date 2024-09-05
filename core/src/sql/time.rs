/**
 * Rust type for
 * [time](https://www.postgresql.org/docs/current/datatype-datetime.html).
 */
#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
pub use time::Time;
#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
pub use time::UtcOffset as Timezone;
/**
 * Rust type for
 * [timetz](https://www.postgresql.org/docs/current/datatype-datetime.html).
 */
#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
pub type TimeTz = (Time, Timezone);

#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
impl crate::ToSql for Time {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::TIME
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L1235
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L1281
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let usecs = self.hour() as i64 * 60 * 60 * 1_000_000
            + self.minute() as i64 * 60 * 1_000_000
            + self.second() as i64 * 1_000_000
            + self.millisecond() as i64 * 1_000
            + self.microsecond() as i64;

        let mut buf = Vec::new();
        crate::to_sql::write_i64(&mut buf, usecs)?;

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
impl crate::FromSql for Time {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L1170
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let format = time::macros::format_description!("[hour]:[minute]:[second]");
        Time::parse(crate::from_sql::not_null(raw)?, &format).map_err(|_| Self::error(ty, raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L1255
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let usec = i64::from_binary(ty, raw)?;

        Ok(Time::MIDNIGHT + time::Duration::microseconds(usec))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
impl crate::entity::Simple for Time {}

#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
impl crate::ToSql for TimeTz {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::TIMETZ
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L2006
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        format!("{}{}", self.0, self.1).to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L2063
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = self.0.to_binary()?.unwrap();

        crate::to_sql::write_i32(&mut buf, -self.1.whole_seconds())?;

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
impl crate::FromSql for TimeTz {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L1971
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let value = crate::from_sql::not_null(raw)?;

        let x = match value.find(['+', '-']) {
            Some(x) => x,
            None => return Err(Self::error(ty, raw)),
        };

        let format = time::macros::format_description!("[hour]:[minute]:[second]");
        let time = Time::parse(&value[0..x], &format).map_err(|_| Self::error(ty, raw))?;

        let mut tz = value[x..].replace(':', "");

        if tz.len() == 3 {
            tz.push_str("00");
        }

        let format = time::macros::format_description!("[offset_hour][offset_minute]");
        let timezone = Timezone::parse(&tz, &format).map_err(|_| Self::error(ty, raw))?;

        Ok((time, timezone))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L2027
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = crate::from_sql::not_null(raw)?;
        let time = crate::from_sql::read_i64(&mut buf)?;
        let zone = crate::from_sql::read_i32(&mut buf)?;

        Ok((
            Time::MIDNIGHT + time::Duration::microseconds(time),
            Timezone::from_whole_seconds(-zone).map_err(|_| Self::error(ty, raw))?,
        ))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
impl crate::entity::Simple for TimeTz {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        time,
        crate::Time,
        [
            ("'00:00:00'", crate::Time::MIDNIGHT),
            ("'01:02:03'", time::macros::time!(01:02:03)),
        ]
    );

    crate::sql_test!(
        timetz,
        crate::TimeTz,
        [
            (
                "'00:00:00+0000'",
                (crate::Time::MIDNIGHT, crate::Timezone::UTC)
            ),
            (
                "'01:02:03+0200'",
                (time::macros::time!(01:02:03), time::macros::offset!(+2))
            ),
        ]
    );
}
