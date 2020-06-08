pub use time::Time;

impl crate::ToSql for Time {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::TIME
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

impl crate::FromSql for Time {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        Time::parse(crate::not_null(raw)?, "%T")
            .map_err(|_| Self::error(ty, "time", raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L1281
     */
    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let usec = i64::from_binary(ty, raw)?;

        Ok(Time::midnight() + time::Duration::microseconds(usec))
    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(time, crate::Time, [
        ("'00:00:00'", crate::Time::midnight()),
        ("'01:02:03'", time::time!(01:02:03)),
    ]);
}
