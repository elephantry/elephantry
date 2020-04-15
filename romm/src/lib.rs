mod connection;
mod entity;
mod projection;
mod model;
mod row;
mod row_structure;

#[cfg(feature = "romm-derive")]
pub use romm_derive::*;

pub use connection::*;
pub use entity::*;
pub use model::*;
pub use projection::*;
pub use row::*;
pub use row_structure::*;

use std::collections::HashMap;

/**
 * Easily create pk argument for where clause, including find_by_pk function
 *
 * pk!(uuid)
 * pk!(&uuid)
 * pk!{ uuid, name }
 * pk!{ "uuid" => uuid, "name" => name }
 */
#[macro_export]
macro_rules! pk {
    ($pk:ident) => {
        $crate::pk!($pk,)
    };

    ($($pk:ident,)+) => {
        $crate::pk!($(
            $pk => $pk,
        )*)
    };

    ($($key:expr => $value:expr,)+) => {{
        let mut hash = std::collections::HashMap::new();

        $(
            hash.insert(stringify!($key), &$value as &dyn postgres::types::ToSql);
        )*

        hash
    }}
}

pub struct Romm
{
    default: String,
    connections: HashMap<String, Connection>,
}

impl Romm
{
    pub fn new() -> Self
    {
        Self {
            default: String::new(),
            connections: HashMap::new(),
        }
    }

    pub fn add_default(self, name: &str, url: &str) -> postgres::Result<Self>
    {
        self.add(name, url, true)
    }

    pub fn add_connection(self, name: &str, url: &str) -> postgres::Result<Self>
    {
        self.add(name, url, false)
    }

    fn add(mut self, name: &str, url: &str, default: bool) -> postgres::Result<Self>
    {
        self.connections.insert(name.to_string(), Connection::new(url)?);

        if default {
            self.set_default(name);
        }

        Ok(self)
    }

    pub fn default(&self) -> Option<&Connection>
    {
        self.connections.get(&self.default)
    }

    pub fn set_default(&mut self, name: &str)
    {
        self.default = name.to_string();
    }

    pub fn get(&self, name: &str) -> Option<&Connection>
    {
        self.connections.get(&name.to_string())
    }
}

impl Default for Romm
{
    fn default() -> Self
    {
        Self::new()
    }
}

#[cfg(test)]
mod test {
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
        let pk = crate::pk! {
            uuid,
            name,
        };

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
