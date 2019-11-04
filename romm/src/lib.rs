use std::collections::HashMap;

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

pub struct Connection
{
    connection: postgres::Connection,
}

impl Connection
{
    pub fn new(url: &str) -> postgres::Result<Self>
    {
        Ok(Self {
            connection: postgres::Connection::connect(url, postgres::TlsMode::None)?,
        })
    }

    pub fn find_all<M>(&self)
        -> postgres::Result<Vec<M::Entity>> where M: Model
    {
        let query = format!(
            "SELECT {} FROM {};",
            M::create_projection(),
            M::RowStructure::relation(),
        );

        let results = self.connection.query(&query, &[])?;

        Ok(results.iter()
            .map(|row| M::create_entity(row))
            .collect()
        )
    }
}

pub trait Model
{
    type Entity: Entity;
    type RowStructure: RowStructure;

    fn default_projection() -> Projection
    {
        Projection::new(&Self::RowStructure::definition())
    }

    fn create_projection() -> Projection
    {
        Self::default_projection()
    }

    fn create_entity(row: postgres::rows::Row<'_>) -> Self::Entity
    {
        let projection = Self::create_projection();
        let mut data = HashMap::<&'static str, (postgres::types::Type, Vec<u8>)>::new();

        for (name, Row {ty, content: _ }) in projection.fields {
             data.insert(name, (ty, row.get_bytes(name).unwrap().to_vec()));
        }

        <Self::Entity as Entity>::from(&data)
    }
}

pub trait Entity
{
    fn from(data: &HashMap<&'static str, (postgres::types::Type, Vec<u8>)>) -> Self;
}

pub trait RowStructure
{
    fn relation() -> &'static str;
    fn primary_key() -> &'static [&'static str];
    fn definition() -> HashMap<&'static str, Row>;
}

#[derive(Clone)]
pub struct Row
{
    pub content: &'static str,
    pub ty: postgres::types::Type,
}

pub struct Projection
{
    fields: HashMap<&'static str, Row>,
}

impl Projection
{
    pub fn new(fields: &HashMap<&'static str, Row>) -> Self
    {
        Self {
            fields: fields.clone(),
        }
    }

    pub fn set_field(mut self, name: &'static str, row: Row) -> Projection
    {
        self.fields.insert(name, row);

        self
    }

    pub fn set_field_type(mut self, name: &str, ty: postgres::types::Type) -> Projection
    {
        if let Some(row) = self.fields.get_mut(name) {
            row.ty = ty;
        }

        self
    }

    pub fn unset_field(mut self, name: &str) -> Projection
    {
        self.fields.remove(name);

        self
    }

    pub fn fields(&self) -> &HashMap<&'static str, Row>
    {
        &self.fields
    }
}

impl std::fmt::Display for Projection
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let s = self.fields.iter()
            .map(|(alias, row)| {
                let content = row.content
                    .replace("\"", "\\\"")
                    .replace("%:", "\"")
                    .replace(":%", "\"");
                format!(r#"{} as "{}""#, content, alias)
            })
            .fold(String::new(), |acc, x| {
                if acc.is_empty() {
                    x
                }
                else {
                    format!("{}, {}", acc, x)
                }
            });

        write!(f, "{}", s)
    }
}

pub struct Session
{
}
