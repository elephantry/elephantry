use crate::RowStructure;
use std::collections::HashMap;

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

    pub fn find_by_pk<M>(&self, pk: &HashMap<&str, &dyn postgres::types::ToSql>)
        -> postgres::Result<Option<M::Entity>> where M: crate::Model
    {
        let (clause, params) = self.pk_clause::<M>(pk);
        let rows = self.find_where::<M>(&clause, &params)?;

        Ok(match rows.get(0) {
            Some(e) => Some(e.clone()),
            None => None,
        })
    }

    pub fn find_all<M>(&self)
        -> postgres::Result<Vec<M::Entity>> where M: crate::Model
    {
        let query = format!(
            "SELECT {} FROM {};",
            M::create_projection(),
            M::RowStructure::relation(),
        );

        let results = self.connection.query(&query, &[])?;

        Ok(results.iter()
            .map(M::create_entity)
            .collect()
        )
    }

    pub fn find_where<M>(&self, clause: &str, params: &[&dyn postgres::types::ToSql])
        -> postgres::Result<Vec<M::Entity>> where M: crate::Model
    {
        let query = format!(
            "SELECT {} FROM {} WHERE {};",
            M::create_projection(),
            M::RowStructure::relation(),
            clause,
        );

        let results = self.connection.query(&query, params)?;

        Ok(results.iter()
            .map(|row| M::create_entity(row))
            .collect()
        )
    }

    pub fn insert_one<M>(&self, entity: &M::Entity)
        -> postgres::Result<M::Entity> where M: crate::Model
    {
        use crate::Entity;

        let projection = M::create_projection();

        let mut row = Vec::new();
        let mut params = Vec::new();
        let mut fields = Vec::new();
        let mut x = 1;

        for field in projection.fields_name() {
            if let Some(value) = entity.get(field) {
                row.push(value);
                params.push(format!("${}", x));
                fields.push(field);
                x += 1;
            }
        }

        let query = format!(
            "INSERT INTO {} ({}) VALUES({}) RETURNING *;",
            M::RowStructure::relation(),
            fields.join(", "),
            params.join(", "),
        );

        let results = self.connection.query(&query, row.as_slice())?;

        Ok(M::create_entity(results.get(0)))
    }

    pub fn update_one<M>(&self, entity: &M::Entity, data: &HashMap<&str, &dyn postgres::types::ToSql>)
        -> postgres::Result<M::Entity> where M: crate::Model
    {
        let pk = M::primary_key(&entity);
        let (clause, mut params) = self.pk_clause::<M>(&pk);
        let mut x = params.len() + 1;
        let mut set = Vec::new();

        for (key, value) in data.iter() {
            set.push(format!("{} = ${}", key, x));
            params.push(value.clone());
            x += 1;
        }

        let query = format!(
            "UPDATE {} SET {} WHERE {} RETURNING *;",
            M::RowStructure::relation(),
            set.join(", "),
            clause,
        );

        let results = self.connection.query(&query, &params)?;

        Ok(M::create_entity(results.get(0)))
    }

    fn pk_clause<'a, M>(&self, pk: &HashMap<&str, &'a dyn postgres::types::ToSql>)
        -> (String, Vec<&'a dyn postgres::types::ToSql>) where M: crate::Model
    {
        let keys: Vec<_> = pk.keys()
            .map(|x| *x)
            .collect();

        if  keys != M::RowStructure::primary_key() {
            panic!("Invalid pk");
        }

        let clause = keys.iter()
            .enumerate()
            .fold(String::new(), |acc, (i, x)| {
                if acc.is_empty() {
                   format!("{} = ${}", x, i + 1)
                }
                else {
                    format!("{} AND {} = ${}", acc, x, i + 1)
                }
            });

        let params: Vec<_> = pk.values()
            .into_iter()
            .map(|e| *e)
            .collect();

        (clause, params)
    }
}
