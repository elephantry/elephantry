use std::collections::HashMap;

pub trait Entity {
    fn from(tuple: &crate::Tuple<'_>) -> Self;
    fn get(&self, field: &str) -> Option<&dyn crate::ToSql>;
}

impl<T: crate::ToSql + crate::FromSql> Entity for T {
    fn from(tuple: &crate::Tuple<'_>) -> T {
        tuple.nth(0)
    }

    fn get(&self, _: &str) -> Option<&dyn crate::ToSql> {
        Some(self)
    }
}

impl<T: crate::FromSql + crate::ToSql, S: std::hash::BuildHasher + Default>
    Entity for HashMap<String, T, S>
{
    fn from(tuple: &crate::Tuple<'_>) -> Self {
        let mut hashmap = HashMap::default();

        for x in 0..tuple.len() {
            let name = tuple.field_name(x).unwrap();
            let value = tuple.nth(x);
            hashmap.insert(name, value);
        }

        hashmap
    }

    fn get(&self, field: &str) -> Option<&dyn crate::ToSql> {
        match self.get(field) {
            Some(value) => Some(value),
            None => None,
        }
    }
}

impl<T: crate::FromSql + crate::ToSql, S: std::hash::BuildHasher + Default>
    Entity for HashMap<usize, T, S>
{
    fn from(tuple: &crate::Tuple<'_>) -> Self {
        let mut hashmap = HashMap::default();

        for x in 0..tuple.len() {
            let value = tuple.nth(x);
            hashmap.insert(x, value);
        }

        hashmap
    }

    fn get(&self, field: &str) -> Option<&dyn crate::ToSql> {
        match self.get(&field.parse::<usize>().unwrap()) {
            Some(value) => Some(value),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn hashmap_str_from_sql() -> crate::Result<()> {
        let elephantry = crate::test::new_conn()?;
        let results: Vec<HashMap<String, i32>> =
            elephantry.query("SELECT 1 as n", &[])?.collect();

        assert_eq!(results[0].get("n"), Some(&1));

        Ok(())
    }

    #[test]
    fn hashmap_usize_from_sql() -> crate::Result<()> {
        let elephantry = crate::test::new_conn()?;
        let results: Vec<HashMap<usize, i32>> =
            elephantry.query("SELECT 1 as n", &[])?.collect();

        assert_eq!(results[0].get(&0), Some(&1));

        Ok(())
    }
}
