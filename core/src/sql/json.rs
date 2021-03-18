impl crate::ToSql for serde_json::value::Value {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::JSON
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        match serde_json::to_string(self) {
            Ok(s) => s.to_sql(),
            Err(err) => Err(self.error("json", Some(&err.to_string()))),
        }
    }
}

impl crate::FromSql for serde_json::value::Value {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        match serde_json::from_str(crate::not_null(raw)?) {
            Ok(json) => Ok(json),
            _ => Err(Self::error(ty, "json", raw)),
        }
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let s = String::from_binary(ty, raw)?;

        match serde_json::from_str(&s) {
            Ok(json) => Ok(json),
            _ => Err(Self::error(ty, "json", raw)),
        }
    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(
        json,
        serde_json::value::Value,
        [("'{\"foo\": \"bar\"}'", serde_json::json!({"foo": "bar"}))]
    );
}
