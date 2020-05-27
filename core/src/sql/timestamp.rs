impl crate::ToSql for chrono::DateTime<chrono::offset::Utc> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMPTZ
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_rfc2822().to_sql()
    }
}

impl crate::FromSql for chrono::DateTime<chrono::offset::Utc> {
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

impl crate::ToSql for chrono::DateTime<chrono::offset::FixedOffset> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMPTZ
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_rfc2822().to_sql()
    }
}

impl crate::FromSql for chrono::DateTime<chrono::offset::FixedOffset> {
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

impl crate::ToSql for chrono::DateTime<chrono::offset::Local> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMPTZ
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F %T").to_string().to_sql()
    }
}

impl crate::FromSql for chrono::DateTime<chrono::offset::Local> {
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

impl crate::ToSql for chrono::NaiveDateTime {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMP
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F %T").to_string().to_sql()
    }
}

impl crate::FromSql for chrono::NaiveDateTime {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        if let Ok(date) =
            chrono::NaiveDateTime::parse_from_str(crate::not_null!(raw), "%F %T")
        {
            return Ok(date);
        }

        match chrono::NaiveDateTime::parse_from_str(crate::not_null!(raw), "%F %T.%f")
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

#[cfg(test)]
mod test {
    mod from {
        crate::from_test!(timestamp, chrono::NaiveDateTime, [
            ("'1970-01-01 00:00:00'", chrono::NaiveDateTime::from_timestamp(0, 0)),
        ]);
    }

    mod to {
        crate::to_test!(timestamp, [
            chrono::NaiveDateTime::from_timestamp(0, 0),
        ]);
    }
}
