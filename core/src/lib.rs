#![warn(rust_2018_idioms)]

pub mod inspect;
pub mod pq;

mod array;
mod config;
mod connection;
mod entity;
mod errors;
mod from_sql;
mod model;
mod pager;
mod pgpass;
mod pool;
mod projection;
mod rows;
mod sql;
mod structure;
mod to_sql;
mod tuple;

pub use array::*;
pub use config::*;
pub use connection::*;
pub use elephantry_derive::*;
pub use entity::*;
pub use errors::*;
pub use from_sql::*;
pub use model::*;
pub use pager::*;
pub use pool::*;
pub use projection::*;
pub use rows::*;
pub use sql::*;
pub use structure::*;
pub use to_sql::*;
pub use tuple::*;

use pgpass::*;

/**
 * Easily create pk argument for where clause, including find_by_pk function
 *
 * pk!(uuid)
 * pk![uuid, name]
 * pk!{uuid => "uuid", name => "name"}
 */
#[macro_export]
macro_rules! pk {
    ($($pk:ident),+ $(,)?) => {
        $crate::pk!($(
            $pk => $pk,
        )*)
    };

    ($($key:expr => $value:expr),+ $(,)?) => {{
        let mut hash = std::collections::HashMap::new();

        $(
            hash.insert(stringify!($key), &$value as &dyn $crate::ToSql);
        )*

        hash
    }}
}

#[cfg(test)]
mod test {
    static INIT: std::sync::Once = std::sync::Once::new();

    #[macro_export]
    macro_rules! sql_test {
        ($sql_type:ident, $rust_type:ty, $tests:expr) => {
            mod $sql_type {
                use std::collections::HashMap;

                #[test]
                fn from_text() -> crate::Result<()> {
                    let conn = crate::test::new_conn();
                    conn.execute("create extension if not exists hstore")?;
                    conn.execute("set lc_monetary to 'en_US.UTF-8';")?;

                    for (value, expected) in &$tests {
                        let result = conn.execute(&format!(
                            "select {}::{} as actual",
                            value,
                            stringify!($sql_type)
                        ))?;
                        assert_eq!(
                            result.get(0).get::<$rust_type>("actual"),
                            *expected
                        );
                    }

                    Ok(())
                }

                #[test]
                fn from_binary() -> crate::Result<()> {
                    let conn = crate::test::new_conn();
                    conn.execute("set lc_monetary to 'en_US.UTF-8';")?;

                    for (value, expected) in &$tests {
                        let result = conn
                            .query::<HashMap<String, $rust_type>>(
                                &format!(
                                    "select {}::{} as actual",
                                    value,
                                    stringify!($sql_type)
                                ),
                                &[],
                            )?;
                        assert_eq!(
                            result.get(0).get("actual").unwrap(),
                            expected
                        );
                    }

                    Ok(())
                }

                #[test]
                fn to() -> crate::Result<()> {
                    let conn = crate::test::new_conn();
                    conn.execute("set lc_monetary to 'en_US.UTF-8';")?;

                    for (_, value) in &$tests {
                        let result = conn.query::<HashMap<String, String>>(
                            &format!("select $1::{}", stringify!($sql_type)),
                            &[value],
                        );
                        assert!(result.is_ok());
                    }

                    Ok(())
                }
            }
        };
    }

    pub fn dsn() -> String {
        std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "host=localhost".to_string())
    }

    pub fn new_conn() -> crate::Pool {
        INIT.call_once(|| {
            pretty_env_logger::init();
        });

        crate::Pool::new(&dsn()).unwrap()
    }

    #[test]
    fn test_pk_one() {
        let uuid = "1234";
        let pk = crate::pk!(uuid);

        assert_eq!(pk.len(), 1);
        assert!(pk.contains_key("uuid"));
    }

    #[test]
    fn test_pk_multi() {
        let uuid = "1234";
        let name = "name";
        let pk = crate::pk![uuid, name,];

        assert_eq!(pk.len(), 2);
        assert!(pk.contains_key("uuid"));
        assert!(pk.contains_key("name"));
    }

    #[test]
    fn test_pk_hash() {
        let pk = crate::pk! {
            uuid => "1234",
            name => "name",
        };

        assert_eq!(pk.len(), 2);
        assert!(pk.contains_key("uuid"));
        assert!(pk.contains_key("name"));
    }
}
