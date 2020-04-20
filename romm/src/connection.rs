use crate::row::Structure;
use std::collections::HashMap;

pub struct Connection
{
    connection: crate::pq::Connection,
}

impl Connection
{
    pub fn new(dsn: &str) -> crate::Result<Self>
    {
        Ok(Self {
            connection: crate::pq::Connection::new(dsn)?,
        })
    }

    pub fn find_by_pk<M>(&self, pk: &HashMap<&str, &dyn crate::pq::ToSql>)
        -> crate::Result<Option<M::Entity>> where M: crate::Model
    {
        let (clause, params) = self.pk_clause::<M>(pk);
        let tuples = self.find_where::<M>(&clause, &params)?;

        Ok(match tuples.get(0) {
            Some(e) => Some(e.clone()),
            None => None,
        })
    }

    pub fn find_all<M>(&self)
        -> crate::Result<Vec<M::Entity>> where M: crate::Model
    {
        let query = format!(
            "SELECT {} FROM {};",
            M::create_projection(),
            M::RowStructure::relation(),
        );

        let results = self.connection.query(&query, &[])?;

        Ok(results.map(|tuple| M::create_entity(&tuple))
            .collect()
        )
    }

    pub fn find_where<M>(&self, clause: &str, params: &[&dyn crate::pq::ToSql])
        -> crate::Result<Vec<M::Entity>> where M: crate::Model
    {
        let query = format!(
            "SELECT {} FROM {} WHERE {};",
            M::create_projection(),
            M::RowStructure::relation(),
            clause,
        );

        let results = self.connection.query(&query, params)?;

        Ok(results.map(|tuple| M::create_entity(&tuple))
            .collect()
        )
    }

    pub fn count_where<M>(&self, clause: &str, params: &[&dyn crate::pq::ToSql])
        -> crate::Result<i64> where M: crate::Model
    {
        let query = format!(
            "SELECT COUNT(*) FROM {} WHERE {};",
            M::RowStructure::relation(),
            clause,
        );

        let results = self.connection.query(&query, params)?;

        Ok(results.get(0).unwrap().get("count")?)
    }

    pub fn exist_where<M>(&self, clause: &str, params: &[&dyn crate::pq::ToSql])
        -> crate::Result<bool> where M: crate::Model
    {
        let query = format!(
            "SELECT EXISTS (SELECT true FROM {} WHERE {}) AS result;",
            M::RowStructure::relation(),
            clause,
        );

        let results = self.connection.query(&query, params)?;

        Ok(results.get(0).unwrap().get("result")?)
    }

    pub fn insert_one<M>(&self, entity: &M::Entity)
        -> crate::Result<M::Entity> where M: crate::Model
    {
        use crate::Entity;

        let projection = M::create_projection();

        let mut tuple = Vec::new();
        let mut params = Vec::new();
        let mut fields = Vec::new();
        let mut x = 1;

        for field in projection.fields_name() {
            if let Some(value) = entity.get(field) {
                tuple.push(value);
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

        let results = self.connection.query(&query, tuple.as_slice())?;

        Ok(M::create_entity(&results.get(0).unwrap()))
    }

    pub fn update_one<M>(&self, entity: &M::Entity, data: &HashMap<&str, &dyn crate::pq::ToSql>)
        -> crate::Result<M::Entity> where M: crate::Model
    {
        let pk = M::primary_key(&entity);

        self.update_by_pk::<M>(&pk, data)
    }

    pub fn update_by_pk<M>(&self, pk: &HashMap<&str, &dyn crate::pq::ToSql>, data: &HashMap<&str, &dyn crate::pq::ToSql>)
        -> crate::Result<M::Entity> where M: crate::Model
    {
        let (clause, mut params) = self.pk_clause::<M>(&pk);
        let mut x = params.len() + 1;
        let mut set = Vec::new();

        for (key, value) in data.iter() {
            set.push(format!("{} = ${}", key, x));
            params.push(*value);
            x += 1;
        }

        let query = format!(
            "UPDATE {} SET {} WHERE {} RETURNING *;",
            M::RowStructure::relation(),
            set.join(", "),
            clause,
        );

        let results = self.connection.query(&query, &params)?;

        Ok(M::create_entity(&results.get(0).unwrap()))
    }

    pub fn delete_one<M>(&self, entity: &M::Entity)
        -> crate::Result<M::Entity> where M: crate::Model
    {
        let pk = M::primary_key(&entity);

        self.delete_by_pk::<M>(&pk)
    }

    pub fn delete_by_pk<M>(&self, pk: &HashMap<&str, &dyn crate::pq::ToSql>)
        -> crate::Result<M::Entity> where M: crate::Model
    {
        let (clause, params) = self.pk_clause::<M>(&pk);

        let results = self.delete_where::<M>(&clause, &params)?;

        Ok(results.get(0).unwrap().clone())
    }

    pub fn delete_where<M>(&self, clause: &str, params: &[&dyn crate::pq::ToSql])
        -> crate::Result<Vec<M::Entity>> where M: crate::Model
    {
        let query = format!(
            "DELETE FROM {} WHERE {} RETURNING *;",
            M::RowStructure::relation(),
            clause,
        );

        let results = self.connection.query(&query, &params)?;

        Ok(
            results.map(|tuple| M::create_entity(&tuple))
                .collect()
        )
    }

    fn pk_clause<'a, M>(&self, pk: &HashMap<&str, &'a dyn crate::pq::ToSql>)
        -> (String, Vec<&'a dyn crate::pq::ToSql>) where M: crate::Model
    {
        let keys: Vec<_> = pk.keys()
            .copied()
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
            .copied()
            .collect();

        (clause, params)
    }
}
