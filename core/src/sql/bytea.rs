#[derive(Clone, Debug, PartialEq)]
pub struct Bytea(Vec<u8>);

impl From<Vec<u8>> for Bytea {
    fn from(vec: Vec<u8>) -> Self {
        Self(vec)
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
        crate::pq::ty::BYTEA
    }

    fn format(&self) -> crate::pq::Format {
        crate::pq::Format::Binary
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        Ok(Some(self.to_vec()))
    }
}

impl crate::FromSql for Bytea {
    fn from_text(
        _: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        let x: &[_] = &['\\', 'x'];
        let string = crate::not_null(raw)?.trim_start_matches(x);
        let mut pos = 0;
        let mut v = Vec::new();

        while pos < string.len() {
            v.push(u8::from_str_radix(&string[pos..pos + 2], 16).unwrap());
            pos += 2;
        }

        Ok(v.into())
    }

    fn from_binary(
        _: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        Ok(crate::not_null(raw)?.to_vec().into())
    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(bytea, crate::Bytea, [(
        "'abcd'",
        crate::Bytea::from(Vec::from("abcd"))
    ),]);
}
