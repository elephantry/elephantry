#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ltxtquery(String);

impl From<String> for Ltxtquery {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Ltxtquery {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

impl ToString for Ltxtquery {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl std::str::FromStr for Ltxtquery {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl std::ops::Deref for Ltxtquery {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Ltxtquery {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl crate::ToSql for Ltxtquery {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::Type {
            descr: "LTXTQUERY - data type for hierarchical tree-like structures",
            name: "ltxtquery",

            ..crate::pq::types::UNKNOWN
        }
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_14_STABLE/contrib/ltree/ltree_io.c#L730
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_14_STABLE/contrib/ltree/ltree_io.c#L781
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        format!("\u{1}{}", self.to_string()).to_binary()
    }
}

impl crate::FromSql for Ltxtquery {
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
        let mut buf = crate::from_sql::not_null(raw)?;

        let _version = crate::from_sql::read_i8(&mut buf)?;
        String::from_binary(ty, Some(buf)).map(|x| x.parse().map_err(crate::Error::Infallible))?
    }
}

impl crate::entity::Simple for Ltxtquery {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        ltxtquery,
        crate::Ltxtquery,
        [(
            "'Europe & Russia@* & !Transportation'",
            crate::Ltxtquery::from("Europe & Russia@* & !Transportation"),
        )]
    );
}
