#[allow(unused)]
#[derive(Debug, Default, PartialEq)]
struct Ltree(Vec<String>);

impl elephantry::ToSql for Ltree {
    fn ty(&self) -> elephantry::pq::Type {
        elephantry::pq::Type {
            descr: "LQUERY - data type for hierarchical tree-like structures",
            name: "lquery",

            ..elephantry::pq::types::UNKNOWN
        }
    }

    fn to_text(&self) -> elephantry::Result<Option<String>> {
        self.0.join(".").to_text()
    }

    fn to_binary(&self) -> elephantry::Result<Option<Vec<u8>>> {
        let mut buf = vec![1];
        buf.extend_from_slice(&self.0.join(".").into_bytes());

        Ok(Some(buf))
    }
}

impl elephantry::FromSql for Ltree {
    fn from_text(ty: &elephantry::pq::Type, raw: Option<&str>) -> elephantry::Result<Self> {
        let s = String::from_text(ty, raw)?;

        let ltree = if s.is_empty() {
            Self::default()
        } else {
            Self(s.split('.').map(ToString::to_string).collect())
        };

        Ok(ltree)
    }

    fn from_binary(ty: &elephantry::pq::Type, raw: Option<&[u8]>) -> elephantry::Result<Self> {
        let mut buf = elephantry::from_sql::not_null(raw)?;

        let _version = elephantry::from_sql::read_i8(&mut buf)?;
        let s = String::from_binary(ty, Some(buf))?;
        Self::from_text(ty, Some(&s))
    }
}

impl elephantry::entity::Simple for Ltree {}

fn main() {}

#[cfg(test)]
mod test {
    elephantry::testing::convertion! {
        fixture: "test",
        sql_type: ltree,
        rust_type: crate::Ltree,
        tests: [
            ("''", crate::Ltree::default()),
            (
                "'Top.Countries.Europe.Russia'",
                crate::Ltree(vec![
                    "Top".to_string(),
                    "Countries".to_string(),
                    "Europe".to_string(),
                    "Russia".to_string()
                ])
            ),
        ],
    }
}
