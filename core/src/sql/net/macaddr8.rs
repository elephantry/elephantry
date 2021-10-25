#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
impl crate::ToSql for macaddr::MacAddr8 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::MACADDR8
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/mac8.c#L242
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/mac8.c#L295
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        Ok(Some(self.into_array().to_vec()))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
impl crate::FromSql for macaddr::MacAddr8 {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/mac8.c#L104
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        crate::not_null(raw)?
            .parse()
            .map_err(|_| Self::error(ty, "macaddr::MacAddr8", raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/mac8.c#L262
     */
    fn from_binary(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = crate::from_sql::not_null(raw)?;

        let mut parts = [0; 8];
        for part in &mut parts {
            *part = crate::from_sql::read_u8(&mut buf)?;
        }

        Ok(parts.into())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
impl crate::entity::Simple for macaddr::MacAddr8 {
}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    static MAC: macaddr::MacAddr8 =
        macaddr::MacAddr8::new(0x08, 0x00, 0x2b, 0x01, 0x02, 0x03, 0x04, 0x05);

    crate::sql_test!(
        macaddr8,
        macaddr::MacAddr8,
        [
            (
                "'08:00:2b:01:02:03:04:05'",
                crate::sql::net::macaddr8::test::MAC
            ),
            (
                "'08-00-2b-01-02-03-04-05'",
                crate::sql::net::macaddr8::test::MAC
            ),
            ("'08002b:0102030405'", crate::sql::net::macaddr8::test::MAC),
            ("'08002b-0102030405'", crate::sql::net::macaddr8::test::MAC),
            (
                "'0800.2b01.0203.0405'",
                crate::sql::net::macaddr8::test::MAC
            ),
            (
                "'0800-2b01-0203-0405'",
                crate::sql::net::macaddr8::test::MAC
            ),
            ("'08002b0102030405'", crate::sql::net::macaddr8::test::MAC),
        ]
    );
}
