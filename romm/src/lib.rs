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

pub struct Romm
{
    connection: Connection,
}

impl Romm
{
    pub fn new(url: &str) -> postgres::Result<Self>
    {
        Ok(Self {
            connection: Connection::new(url)?,
        })
    }

    pub fn get(&self) -> &Connection
    {
        &self.connection
    }
}
