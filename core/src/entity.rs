use std::collections::HashMap;

pub trait Entity: Clone {
    fn from(tuple: &crate::pq::Tuple<'_>) -> Self;
    fn get(&self, field: &str) -> Option<&dyn crate::ToSql>;
}

impl<T: crate::FromSql + crate::ToSql + Clone, S: std::hash::BuildHasher + Default + Clone> Entity for HashMap<String, T, S> {
    fn from(tuple: &crate::pq::Tuple<'_>) -> Self {
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

impl<T: crate::FromSql + crate::ToSql + Clone, S: std::hash::BuildHasher + Default + Clone> Entity for HashMap<usize, T, S> {
    fn from(tuple: &crate::pq::Tuple<'_>) -> Self {
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
    fn hashmap_str_from_sql() {
        let elephantry = crate::test::new_conn();
        let results: Vec<HashMap<String, i32>> =
            elephantry.query("SELECT 1 as n", &[]).unwrap().collect();

        assert_eq!(results[0].get("n"), Some(&1));
    }

    #[test]
    fn hashmap_usize_from_sql() {
        let elephantry = crate::test::new_conn();
        let results: Vec<HashMap<usize, i32>> =
            elephantry.query("SELECT 1 as n", &[]).unwrap().collect();

        assert_eq!(results[0].get(&0), Some(&1));
    }
}
