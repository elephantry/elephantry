impl crate::ToSql for jiff::Zoned {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::TIMESTAMPTZ
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L756
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.strftime("%F %T%z").to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L818
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        self.with_time_zone(jiff::tz::TimeZone::UTC)
            .datetime()
            .to_binary()
    }
}

impl crate::FromSql for jiff::Zoned {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L386
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let s = crate::from_sql::not_null(raw)?;

        jiff::Zoned::strptime("%F %T%z", format!("{s}00")).map_err(|_| Self::error(ty, raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L784
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let civil = jiff::civil::DateTime::from_binary(ty, raw)?;

        civil
            .to_zoned(jiff::tz::TimeZone::UTC)
            .map_err(|_| Self::error(ty, raw))
    }
}

impl crate::entity::Simple for jiff::Zoned {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        timestamptz,
        jiff::Zoned,
        [(
            "'1970-01-01 00:00:00+00'",
            jiff::civil::date(1970, 1, 1)
                .at(0, 0, 0, 0)
                .to_zoned(jiff::tz::TimeZone::UTC)
                .unwrap(),
        )]
    );
}
