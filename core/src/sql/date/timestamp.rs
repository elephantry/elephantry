/**
 * Rust type for [date](https://www.postgresql.org/docs/current/datatype-datetime.html).
 */
pub type Timestamp = chrono::NaiveDateTime;

impl crate::ToSql for Timestamp {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::TIMESTAMP
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L206
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.format("%F %T").to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L265
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let t: chrono::Duration = *self - super::base_datetime()?;

        let mut buf = Vec::new();
        crate::to_sql::write_i64(&mut buf, t.num_microseconds().unwrap())?;

        Ok(Some(buf))
    }
}

impl crate::FromSql for Timestamp {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L143
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        if let Ok(date) = Timestamp::parse_from_str(crate::from_sql::not_null(raw)?, "%F %T") {
            return Ok(date);
        }

        match Timestamp::parse_from_str(crate::from_sql::not_null(raw)?, "%F %T.%f") {
            Ok(date) => Ok(date),
            _ => Err(Self::error(ty, raw)),
        }
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/timestamp.c#L232
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let t = i64::from_binary(ty, raw)?;

        Ok(super::base_datetime()? + chrono::Duration::microseconds(t))
    }
}

impl crate::entity::Simple for Timestamp {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        timestamp,
        crate::Timestamp,
        [(
            "'1970-01-01 00:00:00'",
            chrono::DateTime::from_timestamp(0, 0)
                .unwrap()
                .naive_local(),
        )]
    );
}
