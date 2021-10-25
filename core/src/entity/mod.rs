mod simple;

pub use simple::Simple;

use std::collections::HashMap;

/**
 * Trait to translate SQL row to struct and vice versa.
 *
 * You probably should use the [`Entity`] derive macro instead of writing the
 * impl by yourself.
 *
 * [`Entity`]: derive.Entity.html
 */
pub trait Entity {
    /** Create a new struct from SQL result. */
    fn from(tuple: &crate::Tuple<'_>) -> Self;
    /** Get the value of the field named `field`. */
    fn get(&self, field: &str) -> Option<&dyn crate::ToSql>;
}

impl<T: crate::FromSql + crate::ToSql, S: std::hash::BuildHasher + Default> Entity
    for HashMap<String, T, S>
{
    fn from(tuple: &crate::Tuple<'_>) -> Self {
        let mut hashmap = HashMap::default();

        for x in 0..tuple.len() {
            let name = match tuple.field_name(x) {
                Some(name) => name,
                None => continue,
            };
            let value = tuple.nth(x);
            hashmap.insert(name, value);
        }

        hashmap
    }

    fn get(&self, field: &str) -> Option<&dyn crate::ToSql> {
        self.get(field).map(|x| x as &dyn crate::ToSql)
    }
}

impl<T: crate::FromSql + crate::ToSql, S: std::hash::BuildHasher + Default> Entity
    for HashMap<usize, T, S>
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
        let x = match field.parse::<usize>() {
            Ok(x) => x,
            Err(err) => {
                log::error!("Unable to retreive HashMap field: {}", err);
                return None;
            }
        };

        self.get(&x).map(|x| x as &dyn crate::ToSql)
    }
}

impl<T: Simple> Entity for T {
    fn from(tuple: &crate::Tuple<'_>) -> T {
        tuple.nth(0)
    }

    fn get(&self, _: &str) -> Option<&dyn crate::ToSql> {
        Some(self)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn hashmap_str_from_sql() -> crate::Result {
        let elephantry = crate::test::new_conn()?;
        let results: Vec<HashMap<String, i32>> = elephantry.query("SELECT 1 as n", &[])?.collect();

        assert_eq!(results[0].get("n"), Some(&1));

        Ok(())
    }

    #[test]
    fn hashmap_usize_from_sql() -> crate::Result {
        let elephantry = crate::test::new_conn()?;
        let results: Vec<HashMap<usize, i32>> = elephantry.query("SELECT 1 as n", &[])?.collect();

        assert_eq!(results[0].get(&0), Some(&1));

        Ok(())
    }
}
