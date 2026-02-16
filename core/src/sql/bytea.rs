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

    fn from_hex(s: &str) -> crate::Result<Self> {
        let mut pos = 2;
        let mut v = Vec::new();

        while pos < s.len() {
            v.push(u8::from_str_radix(&s[pos..pos + 2], 16)?);
            pos += 2;
        }

        Ok(v.into())
    }

    fn from_escape(ty: &crate::pq::Type, s: &str) -> crate::Result<Self> {
        let mut pos = 0;
        let mut v = Vec::new();

        while pos < s.len() {
            let c = s.chars().nth(pos).unwrap();

            if c == '\\' {
                let n = u8::from_str_radix(&s[pos + 1..pos + 4], 8)
                    .map_err(|_| <Self as crate::FromSql>::error(ty, Some(s)))?;

                v.push(n);
                pos += 4;
            } else {
                v.push(c as u8);
                pos += 1;
            }
        }

        Ok(v.into())
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

impl std::ops::DerefMut for Bytea {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let s = crate::from_sql::not_null(raw)?;

        if s.starts_with("\\x") {
            Self::from_hex(s)
        } else {
            Self::from_escape(ty, s)
        }
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
    #[test]
    fn bytea() -> crate::Result {
        let tests = [
            ("'abcd'", crate::Bytea::from(Vec::from("abcd"))),
            ("'\\x123456'", crate::Bytea::from(vec![0x12, 0x34, 0x56])),
        ];

        let conn = crate::test::new_conn()?;

        for output in ["escape", "hex"] {
            conn.execute(&format!("set bytea_output = '{output}'"))?;

            crate::test::to_text("bytea", &tests)?;
            crate::test::to_binary("bytea", &tests)?;
            crate::test::from_text("bytea", &tests)?;
            crate::test::from_binary("bytea", &tests)?;
        }

        Ok(())
    }
}
