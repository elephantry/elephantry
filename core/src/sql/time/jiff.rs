impl crate::ToSql for jiff::civil::Time {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::TIME
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L1235
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        format!("{self:.0}").to_text()
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

impl crate::FromSql for jiff::civil::Time {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L1170
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let s = crate::from_sql::not_null(raw)?;

        s.parse().map_err(|_| Self::error(ty, raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L1255
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let usec = i64::from_binary(ty, raw)?;

        Ok(jiff::civil::Time::MIN + jiff::SignedDuration::from_micros(usec))
    }
}

impl crate::entity::Simple for jiff::civil::Time {}

type TimeTz = (jiff::civil::Time, jiff::tz::Offset);

#[cfg(feature = "time")]
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

        crate::to_sql::write_i32(&mut buf, -self.1.seconds())?;

        Ok(Some(buf))
    }
}

#[cfg(feature = "time")]
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

        let time = value[0..x].parse().map_err(|_| Self::error(ty, raw))?;

        let mut tz = value[x..].replace(':', "");

        if tz.len() == 3 {
            tz.push_str("00");
        }

        let offset = parse_offset(&tz).ok_or_else(|| Self::error(ty, raw))?;

        Ok((time, offset))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L2027
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = crate::from_sql::not_null(raw)?;
        let time = crate::from_sql::read_i64(&mut buf)?;
        let zone = crate::from_sql::read_i32(&mut buf)?;

        Ok((
            jiff::civil::Time::midnight() + jiff::SignedDuration::from_micros(time),
            jiff::tz::Offset::from_seconds(-zone).map_err(|_| Self::error(ty, raw))?,
        ))
    }
}

#[cfg(feature = "time")]
impl crate::entity::Simple for TimeTz {}

fn parse_offset(s: &str) -> Option<jiff::tz::Offset> {
    let mut offset = jiff::tz::Offset::ZERO;

    if s.starts_with('-') {
        offset = offset.negate();
    }
    let s = s.trim_start_matches(|c| c == '+' || c == '-');

    let (hours, minutes) = s.split_at(2);
    let span = jiff::Span::new()
        .hours(hours.parse::<i64>().ok()?)
        .minutes(minutes.parse::<i64>().ok()?);

    Some(offset + span)
}

#[cfg(test)]
mod test {
    crate::sql_test!(
        time,
        jiff::civil::Time,
        [
            ("'00:00:00'", jiff::civil::Time::midnight()),
            ("'01:02:03'", jiff::civil::time(1, 2, 3, 0)),
        ]
    );

    crate::sql_test!(
        timetz,
        (jiff::civil::Time, jiff::tz::Offset),
        [
            (
                "'00:00:00+0000'",
                (jiff::civil::Time::midnight(), jiff::tz::Offset::UTC)
            ),
            (
                "'01:02:03+0200'",
                (jiff::civil::time(1, 2, 3, 0), jiff::tz::offset(2),)
            ),
        ]
    );
}
