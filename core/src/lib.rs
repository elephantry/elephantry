#![warn(rust_2018_idioms)]

pub mod inspect;
pub mod pq;

mod array;
mod connection;
mod entity;
mod errors;
mod from_sql;
#[cfg(feature = "date")]
mod interval;
mod model;
mod pager;
mod pool;
mod projection;
mod rows;
mod structure;
mod to_sql;

pub use array::*;
pub use connection::*;
pub use elephantry_derive::*;
pub use entity::*;
pub use errors::*;
pub use from_sql::*;
#[cfg(feature = "date")]
pub use interval::*;
pub use model::*;
pub use pager::*;
pub use pool::*;
pub use projection::*;
pub use rows::*;
pub use structure::*;
pub use to_sql::*;

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
