#![warn(rust_2018_idioms)]

pub mod inspect;
pub mod pq;

mod array;
mod connection;
mod entity;
mod errors;
mod model;
mod projection;
mod structure;

pub use array::*;
pub use connection::*;
pub use entity::*;
pub use errors::*;
pub use loxo_derive::*;
pub use model::*;
pub use projection::*;
pub use structure::*;

use std::collections::HashMap;

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
            hash.insert(stringify!($key), &$value as &dyn $crate::pq::ToSql);
        )*

        hash
    }}
}

pub struct Loxo {
    default: String,
    connections: HashMap<String, Connection>,
}

impl Loxo {
    pub fn new(url: &str) -> Result<Self> {
        Self::default().add_default("default", url)
    }

    pub fn add_default(self, name: &str, url: &str) -> Result<Self> {
        self.add(name, url, true)
    }

    pub fn add_connection(self, name: &str, url: &str) -> Result<Self> {
        self.add(name, url, false)
    }

    fn add(mut self, name: &str, url: &str, default: bool) -> Result<Self> {
        self.connections
            .insert(name.to_string(), Connection::new(url)?);

        if default {
            self.set_default(name);
        }

        Ok(self)
    }

    pub fn get_default(&self) -> Option<&Connection> {
        self.connections.get(&self.default)
    }

    pub fn set_default(&mut self, name: &str) {
        self.default = name.to_string();
    }

    pub fn get(&self, name: &str) -> Option<&Connection> {
        self.connections.get(&name.to_string())
    }
}

impl Default for Loxo {
    fn default() -> Self {
        Self {
            default: String::new(),
            connections: HashMap::new(),
        }
    }
}

impl std::ops::Index<&str> for Loxo {
    type Output = crate::Connection;

    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl std::ops::Deref for Loxo {
    type Target = crate::Connection;

    fn deref(&self) -> &Self::Target {
        self.get_default().unwrap()
    }
}


#[cfg(test)]
mod test {
    static INIT: std::sync::Once = std::sync::Once::new();

    pub fn dsn() -> String {
        std::env::var("PQ_DSN").unwrap_or_else(|_| "host=localhost".to_string())
    }

    pub fn new_conn() -> crate::Loxo {
        INIT.call_once(|| {
            pretty_env_logger::init();
        });

        crate::Loxo::new(&dsn()).unwrap()
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
