impl crate::ToSql for chrono::Date<chrono::offset::Utc> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMPTZ
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F").to_string().to_sql()
    }
}

impl crate::FromSql for chrono::Date<chrono::offset::Utc> {
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

impl crate::ToSql for chrono::Date<chrono::offset::FixedOffset> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMPTZ
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F").to_string().to_sql()
    }
}

impl crate::FromSql for chrono::Date<chrono::offset::FixedOffset> {
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

impl crate::ToSql for chrono::Date<chrono::offset::Local> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMPTZ
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F").to_string().to_sql()
    }
}

impl crate::FromSql for chrono::Date<chrono::offset::Local> {
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

impl crate::ToSql for chrono::NaiveDate {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::TIMESTAMP
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F").to_string().to_sql()
    }
}

impl crate::FromSql for chrono::NaiveDate {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        match chrono::NaiveDate::parse_from_str(crate::not_null!(raw), "%F") {
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

#[cfg(test)]
mod test {
    mod from {
        crate::from_test!(date, chrono::NaiveDate, [
            ("'1970-01-01'", chrono::NaiveDate::from_ymd(1970, 01, 01)),
            ("'2010-01-01'", chrono::NaiveDate::from_ymd(2010, 01, 01)),
            ("'2100-12-30'", chrono::NaiveDate::from_ymd(2100, 12, 30)),
        ]);
    }

    mod to {
        crate::to_test!(date, [
            chrono::NaiveDate::from_ymd(1970, 01, 01),
            chrono::NaiveDate::from_ymd(2010, 01, 01),
            chrono::NaiveDate::from_ymd(2100, 12, 30),
        ]);
    }
}
