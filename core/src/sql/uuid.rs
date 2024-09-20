/**
 * Rust type for [uuid](https://www.postgresql.org/docs/current/datatype-uuid.html).
 */
#[cfg_attr(docsrs, doc(cfg(feature = "uuid")))]
pub type Uuid = uuid::Uuid;

#[cfg_attr(docsrs, doc(cfg(feature = "uuid")))]
impl crate::ToSql for Uuid {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::UUID
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/uuid.c#L42
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/uuid.c#L141
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        Ok(Some(self.as_bytes().to_vec()))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "uuid")))]
impl crate::FromSql for Uuid {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/uuid.c#L53
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        match Uuid::parse_str(crate::from_sql::not_null(raw)?) {
            Ok(uuid) => Ok(uuid),
            _ => Err(Self::error(ty, raw)),
        }
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/uuid.c#L152
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let buf = crate::from_sql::not_null(raw)?;

        if buf.len() != 16 {
            return Err(Self::error(ty, raw));
        }

        let mut bytes = [0; 16];
        bytes.copy_from_slice(buf);

        Ok(Uuid::from_bytes(bytes))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "uuid")))]
impl crate::entity::Simple for Uuid {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        uuid,
        crate::Uuid,
        [(
            "'12edd47f-e2fc-44eb-9419-1995dfb6725d'",
            crate::Uuid::parse_str("12edd47f-e2fc-44eb-9419-1995dfb6725d").unwrap()
        )]
    );
}
