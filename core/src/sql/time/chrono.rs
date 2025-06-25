#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
pub type TimeTz = (chrono::NaiveTime, chrono::FixedOffset);

#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl crate::ToSql for chrono::NaiveTime {
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
        use chrono::Timelike as _;

        let usecs = self.hour() as i64 * 60 * 60 * 1_000_000
            + self.minute() as i64 * 60 * 1_000_000
            + self.second() as i64 * 1_000_000
            + self.nanosecond() as i64 / 1_000;

        let mut buf = Vec::new();
        crate::to_sql::write_i64(&mut buf, usecs)?;

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl crate::FromSql for chrono::NaiveTime {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L1170
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        Self::parse_from_str(crate::from_sql::not_null(raw)?, "%H:%M:%S")
            .map_err(|_| Self::error(ty, raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L1255
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let usec = i64::from_binary(ty, raw)?;

        Ok(Self::MIN + std::time::Duration::from_micros(usec as u64))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl crate::entity::Simple for chrono::NaiveTime {}

#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
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

        crate::to_sql::write_i32(&mut buf, self.1.utc_minus_local())?;

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl crate::FromSql for TimeTz {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L1971
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let value = crate::from_sql::not_null(raw)?;

        let x = value.find(['+', '-']).ok_or_else(|| Self::error(ty, raw))?;

        let time = chrono::NaiveTime::parse_from_str(&value[0..x], "%H:%M:%S")
            .map_err(|_| Self::error(ty, raw))?;

        let mut tz = value[x..].to_string();

        if tz.len() == 3 {
            tz.push_str("00");
        }

        let timezone = std::str::FromStr::from_str(&tz).map_err(|_| Self::error(ty, raw))?;

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
            chrono::NaiveTime::MIN + std::time::Duration::from_micros(time as u64),
            chrono::FixedOffset::west_opt(zone).ok_or_else(|| Self::error(ty, raw))?,
        ))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl crate::entity::Simple for TimeTz {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        time,
        chrono::NaiveTime,
        [
            ("'00:00:00'", chrono::NaiveTime::MIN),
            (
                "'01:02:03'",
                chrono::NaiveTime::from_hms_opt(1, 2, 3).unwrap()
            ),
        ]
    );

    crate::sql_test!(
        timetz,
        (chrono::NaiveTime, chrono::FixedOffset),
        [
            (
                "'00:00:00+0000'",
                (
                    chrono::NaiveTime::MIN,
                    chrono::FixedOffset::east_opt(0).unwrap()
                )
            ),
            (
                "'01:02:03+0200'",
                (
                    chrono::NaiveTime::from_hms_opt(1, 2, 3).unwrap(),
                    chrono::FixedOffset::east_opt(2 * 60 * 60).unwrap()
                )
            ),
        ]
    );
}
