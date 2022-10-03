mod lquery;

pub use lquery::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Ltree(Vec<String>);

impl From<Vec<String>> for Ltree {
    fn from(v: Vec<String>) -> Self {
        Self(v)
    }
}

impl From<Ltree> for Vec<String> {
    fn from(ltree: Ltree) -> Self {
        ltree.0
    }
}

impl ToString for Ltree {
    fn to_string(&self) -> String {
        self.0.join(".")
    }
}

impl std::str::FromStr for Ltree {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ltree = if s.is_empty() {
            Self::default()
        } else {
            Self(s.split('.').map(ToString::to_string).collect())
        };

        Ok(ltree)
    }
}

impl std::ops::Deref for Ltree {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Ltree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl crate::ToSql for Ltree {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::Type {
            descr: "LTREE - data type for hierarchical tree-like structures",
            name: "ltree",

            ..crate::pq::types::UNKNOWN
        }
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_14_STABLE/contrib/ltree/ltree_io.c#L172
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_14_STABLE/contrib/ltree/ltree_io.c#L223
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        format!("\u{1}{}", self.to_string()).to_binary()
    }
}

impl crate::FromSql for Ltree {
    /*
     * https://github.com/postgres/postgres/blob/REL_14_STABLE/contrib/ltree/ltree_io.c#L181
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        String::from_text(ty, raw).map(|x| x.parse().map_err(crate::Error::Infallible))?
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_14_STABLE/contrib/ltree/ltree_io.c#L223
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = crate::not_null(raw)?;

        let _version = crate::from_sql::read_i8(&mut buf)?;
        String::from_binary(ty, Some(buf)).map(|x| x.parse().map_err(crate::Error::Infallible))?
    }
}

impl crate::entity::Simple for Ltree {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        ltree,
        crate::Ltree,
        [
            ("''", crate::Ltree::default()),
            (
                "'Top.Countries.Europe.Russia'",
                crate::Ltree::from(vec![
                    "Top".to_string(),
                    "Countries".to_string(),
                    "Europe".to_string(),
                    "Russia".to_string()
                ])
            ),
        ]
    );
}
