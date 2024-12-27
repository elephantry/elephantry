mod timestamp;
mod timestamptz;

fn base_date() -> jiff::civil::Date {
    jiff::civil::date(2000, 1, 1)
}

fn base_datetime() -> jiff::civil::DateTime {
    base_date().at(0, 0, 0, 0)
}

impl crate::ToSql for jiff::civil::Date {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::DATE
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L179
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.strftime("%F").to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L226
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let t = *self - base_date();

        let mut buf = Vec::new();
        crate::to_sql::write_i32(&mut buf, t.get_days())?;

        Ok(Some(buf))
    }
}

impl crate::FromSql for jiff::civil::Date {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L204
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        match jiff::civil::Date::strptime("%F", crate::from_sql::not_null(raw)?) {
            Ok(date) => Ok(date),
            _ => Err(Self::error(ty, raw)),
        }
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/date.c#L114
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        use jiff::ToSpan as _;

        let t = i32::from_binary(ty, raw)?;

        Ok(base_date() + t.days())
    }
}

impl crate::entity::Simple for jiff::civil::Date {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        date,
        jiff::civil::Date,
        [
            ("'1970-01-01'", jiff::civil::Date::new(1970, 1, 1).unwrap()),
            ("'2010-01-01'", jiff::civil::Date::new(2010, 1, 1).unwrap()),
            (
                "'2100-12-30'",
                jiff::civil::Date::new(2100, 12, 30).unwrap()
            ),
        ]
    );
}
