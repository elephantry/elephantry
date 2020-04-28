use crate::Structure;
use std::collections::HashMap;

pub struct Connection {
    connection: crate::pq::Connection,
}

impl Connection {
    pub fn new(dsn: &str) -> crate::Result<Self> {
        Ok(Self {
            connection: crate::pq::Connection::new(dsn)?,
        })
    }

    pub fn model<'a, M>(&'a self) -> M
    where
        M: crate::Model<'a>,
    {
        M::new(self)
    }

    pub fn execute(
        &self,
        query: &str
    ) -> crate::Result<crate::pq::Result> {
        self.connection.execute(&query)
    }

    pub fn query<E: crate::Entity>(
        &self,
        query: &str,
        params: &[&dyn crate::pq::ToSql],
    ) -> crate::Result<Vec<E>> {
        let tuples = self.connection.query(&query, params)?;
        let mut results = Vec::with_capacity(tuples.len());

        for tuple in &tuples {
            results.push(E::from(&tuple))
        }

        Ok(results)
    }

    pub fn find_by_pk<'a, M>(
        &self,
        pk: &HashMap<&str, &dyn crate::pq::ToSql>,
    ) -> crate::Result<Option<M::Entity>>
    where
        M: crate::Model<'a>,
    {
        let (clause, params) = self.pk_clause::<M>(pk);
        let tuples = self.find_where::<M>(&clause, &params, None)?;

        Ok(match tuples.get(0) {
            Some(e) => Some(e.clone()),
            None => None,
        })
    }

    pub fn find_all<'a, M>(&self, suffix: Option<&str>) -> crate::Result<Vec<M::Entity>>
    where
        M: crate::Model<'a>,
    {
        let query = format!(
            "SELECT {} FROM {} {};",
            M::create_projection(),
            M::Structure::relation(),
            suffix.unwrap_or_default(),
        );

        self.query(&query, &[])
    }

    pub fn find_where<'a, M>(
        &self,
        clause: &str,
        params: &[&dyn crate::pq::ToSql],
        suffix: Option<&str>,
    ) -> crate::Result<Vec<M::Entity>>
    where
        M: crate::Model<'a>,
    {
        let query = format!(
            "SELECT {} FROM {} WHERE {} {};",
            M::create_projection(),
            M::Structure::relation(),
            clause,
            suffix.unwrap_or_default(),
        );

        self.query(&query, params)
    }

    pub fn count_where<'a, M>(
        &self,
        clause: &str,
        params: &[&dyn crate::pq::ToSql],
    ) -> crate::Result<i64>
    where
        M: crate::Model<'a>,
    {
        let query = format!(
            "SELECT COUNT(*) FROM {} WHERE {};",
            M::Structure::relation(),
            clause,
        );

        let results = self.connection.query(&query, params)?;

        Ok(results.get(0).try_get("count")?)
    }

    pub fn exist_where<'a, M>(
        &self,
        clause: &str,
        params: &[&dyn crate::pq::ToSql],
    ) -> crate::Result<bool>
    where
        M: crate::Model<'a>,
    {
        let query = format!(
            "SELECT EXISTS (SELECT true FROM {} WHERE {}) AS result;",
            M::Structure::relation(),
            clause,
        );

        let results = self.connection.query(&query, params)?;

        Ok(results.get(0).try_get("result")?)
    }

    pub fn insert_one<'a, M>(&self, entity: &M::Entity) -> crate::Result<M::Entity>
    where
        M: crate::Model<'a>,
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
            M::Structure::relation(),
            fields.join(", "),
            params.join(", "),
        );

        let results = self.connection.query(&query, tuple.as_slice())?;

        Ok(M::create_entity(&results.get(0)))
    }

    pub fn update_one<'a, M>(
        &self,
        pk: &HashMap<&str, &dyn crate::pq::ToSql>,
        entity: &M::Entity
    ) -> crate::Result<M::Entity>
    where
        M: crate::Model<'a>,
    {
        use crate::Entity;

        let projection = M::create_projection();
        let mut data = HashMap::new();

        for field in projection.fields_name() {
            if let Some(value) = entity.get(field) {
                data.insert(field.to_string(), value);
            }
        }

        self.update_by_pk::<M>(&pk, &data)
    }

    pub fn update_by_pk<'a, M>(
        &self,
        pk: &HashMap<&str, &dyn crate::pq::ToSql>,
        data: &HashMap<String, &dyn crate::pq::ToSql>,
    ) -> crate::Result<M::Entity>
    where
        M: crate::Model<'a>,
    {
        let (clause, mut params) = self.pk_clause::<M>(&pk);
        let mut x = params.len() + 1;
        let mut set = Vec::new();
        let projection = M::create_projection();

        for (key, value) in data.iter() {
            if projection.has_field(key) {
                set.push(format!("{} = ${}", key, x));
                params.push(*value);
                x += 1;
            }
        }

        let query = format!(
            "UPDATE {} SET {} WHERE {} RETURNING *;",
            M::Structure::relation(),
            set.join(", "),
            clause,
        );

        let results = self.connection.query(&query, &params)?;

        Ok(M::create_entity(&results.get(0)))
    }

    pub fn delete_one<'a, M>(&self, entity: &M::Entity) -> crate::Result<M::Entity>
    where
        M: crate::Model<'a>,
    {
        let pk = M::primary_key(&entity);

        self.delete_by_pk::<M>(&pk)
    }

    pub fn delete_by_pk<'a, M>(
        &self,
        pk: &HashMap<&str, &dyn crate::pq::ToSql>,
    ) -> crate::Result<M::Entity>
    where
        M: crate::Model<'a>,
    {
        let (clause, params) = self.pk_clause::<M>(&pk);

        let results = self.delete_where::<M>(&clause, &params)?;

        Ok(results.get(0).unwrap().clone())
    }

    pub fn delete_where<'a, M>(
        &self,
        clause: &str,
        params: &[&dyn crate::pq::ToSql],
    ) -> crate::Result<Vec<M::Entity>>
    where
        M: crate::Model<'a>,
    {
        let query = format!(
            "DELETE FROM {} WHERE {} RETURNING *;",
            M::Structure::relation(),
            clause,
        );

        self.query(&query, &params)
    }

    fn pk_clause<'a, 'b, M>(
        &self,
        pk: &HashMap<&str, &'b dyn crate::pq::ToSql>,
    ) -> (String, Vec<&'b dyn crate::pq::ToSql>)
    where
        M: crate::Model<'a>,
    {
        let keys: Vec<_> = pk.keys().copied().collect();

        if keys != M::Structure::primary_key() {
            panic!("Invalid pk");
        }

        let definition = M::Structure::definition();

        let clause = keys.iter().enumerate().fold(String::new(), |acc, (i, x)| {
            let field = match definition.get(x) {
                Some(field) => field
                    .replace("\"", "\\\"")
                    .replace("%:", "\"")
                    .replace(":%", "\""),
                None => x.to_string(),
            };

            if acc.is_empty() {
                format!("{} = ${}", field, i + 1)
            } else {
                format!("{} AND {} = ${}", acc, field, i + 1)
            }
        });

        let params: Vec<_> = pk.values().copied().collect();

        (clause, params)
    }
}
