#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lquery(String);

impl From<String> for Lquery {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Lquery {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

impl ToString for Lquery {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl std::str::FromStr for Lquery {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl std::ops::Deref for Lquery {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Lquery {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl crate::ToSql for Lquery {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::Type {
            descr: "LQUERY - data type for hierarchical tree-like structures",
            name: "lquery",

            ..crate::pq::types::UNKNOWN
        }
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_14_STABLE/contrib/ltree/ltree_io.c#L730
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_14_STABLE/contrib/ltree/ltree_io.c#L781
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        format!("\u{1}{}", self.to_string()).to_binary()
    }
}

impl crate::FromSql for Lquery {
    /*
     * https://github.com/postgres/postgres/blob/REL_14_STABLE/contrib/ltree/ltree_io.c#L739
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        String::from_text(ty, raw).map(|x| x.parse().map_err(crate::Error::Infallible))?
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_14_STABLE/contrib/ltree/ltree_io.c#L756
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = crate::not_null(raw)?;

        let _version = crate::from_sql::read_i8(&mut buf)?;
        String::from_binary(ty, Some(buf)).map(|x| x.parse().map_err(crate::Error::Infallible))?
    }
}

impl crate::entity::Simple for Lquery {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        lquery,
        crate::Lquery,
        [("'*.foo.*'", crate::Lquery::from("*.foo.*"))]
    );
}
