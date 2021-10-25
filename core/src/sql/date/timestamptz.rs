#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
impl crate::ToSql for chrono::DateTime<chrono::Utc> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::TIMESTAMPTZ
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L756
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F %T%z").to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L818
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        self.naive_utc().to_binary()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
impl crate::FromSql for chrono::DateTime<chrono::Utc> {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L386
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let ts = chrono::DateTime::<chrono::offset::FixedOffset>::from_text(ty, raw)?;

        Ok(ts.with_timezone(&chrono::Utc))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L784
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let naive = chrono::NaiveDateTime::from_binary(ty, raw)?;
        Ok(chrono::DateTime::from_utc(naive, chrono::Utc))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
impl crate::entity::Simple for chrono::DateTime<chrono::Utc> {
}

#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
impl crate::ToSql for chrono::DateTime<chrono::offset::FixedOffset> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::TIMESTAMPTZ
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F %T%z").to_string().to_text()
    }

    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        self.naive_utc().to_binary()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
impl crate::FromSql for chrono::DateTime<chrono::offset::FixedOffset> {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        chrono::DateTime::parse_from_str(crate::not_null(raw)?, "%F %T%#z")
            .map_err(|_| Self::error(ty, raw))
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let utc = chrono::DateTime::<chrono::Utc>::from_binary(ty, raw)?;
        Ok(utc.with_timezone(&chrono::offset::FixedOffset::east(0)))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
impl crate::entity::Simple for chrono::DateTime<chrono::offset::FixedOffset> {
}

#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
impl crate::ToSql for chrono::DateTime<chrono::offset::Local> {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::TIMESTAMPTZ
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.format("%F %T").to_string().to_text()
    }

    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        self.naive_utc().to_binary()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
impl crate::FromSql for chrono::DateTime<chrono::offset::Local> {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let utc = chrono::DateTime::<chrono::Utc>::from_text(ty, raw)?;
        Ok(utc.with_timezone(&chrono::offset::Local))
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let utc = chrono::DateTime::<chrono::Utc>::from_binary(ty, raw)?;
        Ok(utc.with_timezone(&chrono::offset::Local))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
impl crate::entity::Simple for chrono::DateTime<chrono::offset::Local> {
}

#[cfg(test)]
mod test {
    crate::sql_test!(
        timestamptz,
        chrono::DateTime<chrono::Utc>,
        [(
            "'1970-01-01 00:00:00+00'",
            chrono::DateTime::<chrono::Utc>::from_utc(
                chrono::NaiveDateTime::from_timestamp(0, 0),
                chrono::Utc
            ),
        )]
    );
}
