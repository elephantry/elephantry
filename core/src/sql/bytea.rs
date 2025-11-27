/**
 * Rust type for [bytea](https://www.postgresql.org/docs/current/datatype-binary.html).
 */
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Bytea(Vec<u8>);

impl Bytea {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl From<Vec<u8>> for Bytea {
    fn from(vec: Vec<u8>) -> Self {
        Self(vec)
    }
}

impl From<&[u8]> for Bytea {
    fn from(vec: &[u8]) -> Self {
        Self(vec.to_vec())
    }
}

impl std::ops::Deref for Bytea {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl crate::ToSql for Bytea {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::BYTEA
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varlena.c#L277
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        let data = String::from_utf8(self.0.clone())?;

        Ok(Some(data))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varlena.c#L445
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        Ok(Some(self.to_vec()))
    }
}

impl crate::FromSql for Bytea {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varlena.c#L373
     */
    fn from_text(_: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let x: &[_] = &['\\', 'x'];
        let string = crate::from_sql::not_null(raw)?.trim_start_matches(x);
        let mut pos = 0;
        let mut v = Vec::new();

        while pos < string.len() {
            v.push(u8::from_str_radix(&string[pos..pos + 2], 16)?);
            pos += 2;
        }

        Ok(v.into())
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varlena.c#L464
     */
    fn from_binary(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        Ok(crate::from_sql::not_null(raw)?.to_vec().into())
    }
}

impl crate::entity::Simple for Bytea {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        bytea,
        crate::Bytea,
        [("'abcd'", crate::Bytea::from(Vec::from("abcd")))]
    );
}
