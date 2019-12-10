mod connection;
mod entity;
mod projection;
mod model;
mod row;
mod row_structure;

pub use connection::*;
pub use entity::*;
pub use model::*;
pub use projection::*;
pub use row::*;
pub use row_structure::*;

use std::collections::HashMap;

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
