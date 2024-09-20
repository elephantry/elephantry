/**
 * Rust type for [macaddr](https://www.postgresql.org/docs/current/datatype-net-types.html).
 */
#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
pub type MacAddr = macaddr::MacAddr6;

#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
impl crate::ToSql for MacAddr {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::MACADDR
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/mac.c#L121
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/mac.c#L161
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        Ok(Some(self.into_array().to_vec()))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
impl crate::FromSql for MacAddr {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/mac.c#L56
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        crate::from_sql::not_null(raw)?
            .parse()
            .map_err(|_| Self::error(ty, raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/mac.c#L140
     */
    fn from_binary(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = crate::from_sql::not_null(raw)?;

        let mut parts = [0; 6];
        for part in &mut parts {
            *part = crate::from_sql::read_u8(&mut buf)?;
        }

        Ok(parts.into())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
impl crate::entity::Simple for MacAddr {}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    static MAC: crate::MacAddr = crate::MacAddr::new(0x08, 0x00, 0x2b, 0x01, 0x02, 0x03);

    crate::sql_test!(
        Macaddr,
        crate::MacAddr,
        [
            ("'08:00:2b:01:02:03'", crate::sql::net::macaddr::test::MAC),
            ("'08-00-2b-01-02-03'", crate::sql::net::macaddr::test::MAC),
            ("'08002b:010203'", crate::sql::net::macaddr::test::MAC),
            ("'08002b-010203'", crate::sql::net::macaddr::test::MAC),
            ("'0800.2b01.0203'", crate::sql::net::macaddr::test::MAC),
            ("'0800-2b01-0203'", crate::sql::net::macaddr::test::MAC),
            ("'08002b010203'", crate::sql::net::macaddr::test::MAC),
        ]
    );
}
