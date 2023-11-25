#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
impl crate::ToSql for serde_json::Value {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::JSON
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/json.c#L246
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/json.c#L258
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_binary()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
impl crate::FromSql for serde_json::Value {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/json.c#L228
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        match serde_json::from_str(crate::not_null(raw)?) {
            Ok(json) => Ok(json),
            _ => Err(Self::error(ty, raw)),
        }
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/json.c#L272
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let s = String::from_binary(ty, raw)?;

        match serde_json::from_str(&s) {
            Ok(json) => Ok(json),
            _ => Err(Self::error(ty, raw)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
impl crate::entity::Simple for serde_json::Value {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        json,
        serde_json::Value,
        [("'{\"foo\": \"bar\"}'", serde_json::json!({"foo": "bar"}))]
    );
}
