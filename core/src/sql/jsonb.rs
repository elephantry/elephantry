/**
 * Rust type for jsonb.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct Jsonb(serde_json::Value);

impl From<serde_json::Value> for Jsonb {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for Jsonb {
    type Target = serde_json::Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Jsonb {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
impl crate::ToSql for Jsonb {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::JSONB
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/jsonb.c#L132
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.0.to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/jsonb.c#L148
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();

        crate::to_sql::write_i8(&mut buf, 1)?;
        buf.append(&mut self.0.to_binary()?.unwrap());

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
impl crate::FromSql for Jsonb {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/jsonb.c#L97
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let value = serde_json::Value::from_text(ty, raw)?;

        Ok(Self::from(value))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/jsonb.c#L113
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = crate::not_null(raw)?;

        let _version = crate::from_sql::read_i8(&mut buf)?;
        let value = serde_json::Value::from_binary(ty, Some(buf))?;

        Ok(Self::from(value))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
impl crate::entity::Simple for Jsonb {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        jsonb,
        crate::Jsonb,
        [(
            "'{\"foo\": \"bar\"}'",
            crate::Jsonb::from(serde_json::json!({"foo": "bar"}))
        )]
    );
}
