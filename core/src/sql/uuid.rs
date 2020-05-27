impl crate::ToSql for uuid::Uuid {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::UUID
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

impl crate::FromSql for uuid::Uuid {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        match uuid::Uuid::parse_str(&crate::not_null!(raw)) {
            Ok(uuid) => Ok(uuid),
            _ => Err(Self::error(ty, "uuid", raw)),
        }
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        todo!();
    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(uuid, uuid::Uuid, [
        ("'12edd47f-e2fc-44eb-9419-1995dfb6725d'", uuid::Uuid::parse_str("12edd47f-e2fc-44eb-9419-1995dfb6725d").unwrap()),
    ]);
}
