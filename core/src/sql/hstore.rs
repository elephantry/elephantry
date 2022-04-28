use std::collections::HashMap;

/**
 * Rust type for [hstore](https://www.postgresql.org/docs/current/hstore.html).
 */
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Hstore(HashMap<String, Option<String>>);

impl Hstore {
    pub fn new() -> Self {
        Self::default()
    }

    fn read_string(buf: &mut &[u8]) -> crate::Result<Option<String>> {
        let len = crate::from_sql::read_i32(buf)?;

        let s = if len < 0 {
            None
        } else {
            let mut vec = Vec::new();
            for _ in 0..len {
                vec.push(crate::from_sql::read_u8(buf)?);
            }

            Some(String::from_utf8(vec)?)
        };

        Ok(s)
    }
}

impl std::ops::DerefMut for Hstore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for Hstore {
    type Target = HashMap<String, Option<String>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl crate::ToSql for crate::Hstore {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::UNKNOWN
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/contrib/hstore/hstore_io.c#L407
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut vec = Vec::new();

        for (key, value) in self.iter() {
            let v = value
                .as_ref()
                .map(|x| format!("\"{x}\""))
                .unwrap_or_else(|| "NULL".to_string());

            vec.push(format!("\"{key}\"=>{v}"));
        }

        vec.join(", ").to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/contrib/hstore/hstore_io.c#L1226
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();

        crate::to_sql::write_i32(&mut buf, self.len() as i32)?;

        for (key, value) in self.iter() {
            let mut k = key.to_text()?.unwrap();
            k.pop();
            crate::to_sql::write_i32(&mut buf, k.len() as i32)?;
            buf.append(&mut k);

            if let Some(mut v) = value.to_text()? {
                v.pop();
                crate::to_sql::write_i32(&mut buf, v.len() as i32)?;
                buf.append(&mut v);
            } else {
                crate::to_sql::write_i32(&mut buf, -1)?;
            }
        }

        Ok(Some(buf))
    }
}

impl crate::FromSql for Hstore {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/contrib/hstore/hstore_io.c#L1155
     */
    fn from_text(_: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        lazy_static::lazy_static! {
            static ref REGEX: regex::Regex = regex::Regex::new(
                "\"(?P<key>.*?)\"=>(\"(?P<value>.*?)\"|(?P<null>NULL))",
            ).unwrap();
        }

        let mut hstore = Self::new();

        for capture in REGEX.captures_iter(crate::not_null(raw)?) {
            let key = capture.name("key").unwrap().as_str().to_string();
            let value = if capture.name("null").is_some() {
                None
            } else {
                Some(capture.name("value").unwrap().as_str().to_string())
            };
            hstore.insert(key, value);
        }

        Ok(hstore)
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/contrib/hstore/hstore_io.c#L427
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut hstore = Self::new();
        let mut buf = crate::from_sql::not_null(raw)?;
        let count = crate::from_sql::read_i32(&mut buf)?;

        for _ in 0..count {
            let key = Self::read_string(&mut buf)?.ok_or_else(|| Self::error(ty, raw))?;
            let value = Self::read_string(&mut buf)?;

            hstore.insert(key, value);
        }

        Ok(hstore)
    }
}

impl crate::entity::Simple for Hstore {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        hstore,
        crate::Hstore,
        [("'a=>1, b => 2, c=>null'", {
            let mut hstore = crate::Hstore::new();
            hstore.insert("a".to_string(), Some("1".to_string()));
            hstore.insert("b".to_string(), Some("2".to_string()));
            hstore.insert("c".to_string(), None);

            hstore
        })]
    );
}
