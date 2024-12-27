impl crate::ToSql for jiff::civil::DateTime {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::TIMESTAMP
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L206
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.strftime("%F %T").to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L265
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let t = *self - super::base_datetime();

        let mut buf = Vec::new();
        crate::to_sql::write_i64(&mut buf, t.total(jiff::Unit::Microsecond)? as i64)?;

        Ok(Some(buf))
    }
}

impl crate::FromSql for jiff::civil::DateTime {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L143
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        if let Ok(date) = jiff::civil::DateTime::strptime("%F %T", crate::from_sql::not_null(raw)?)
        {
            return Ok(date);
        }

        match jiff::civil::DateTime::strptime("%F %T.%f", crate::from_sql::not_null(raw)?) {
            Ok(date) => Ok(date),
            _ => Err(Self::error(ty, raw)),
        }
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L232
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        use jiff::ToSpan as _;

        let t = i64::from_binary(ty, raw)?;

        Ok(super::base_datetime() + t.microseconds())
    }
}

impl crate::entity::Simple for jiff::civil::DateTime {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        timestamp,
        jiff::civil::DateTime,
        [(
            "'1970-01-01 00:00:00'",
            jiff::civil::date(1970, 1, 1).at(0, 0, 0, 0)
        )]
    );
}
