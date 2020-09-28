use crate::Structure;
use std::collections::HashMap;
use std::convert::TryInto;

/**
 * A connection to a database.
 */
#[derive(Debug)]
pub struct Connection {
    connection: std::sync::Mutex<libpq::Connection>,
}

extern "C" fn notice_processor(
    _arg: *mut std::ffi::c_void,
    message: *const i8,
) {
    let message = unsafe { std::ffi::CStr::from_ptr(message) };

    log::info!("{}", message.to_str().unwrap().trim());
}

impl Connection {
    pub fn new(dsn: &str) -> crate::Result<Self> {
        let connection = match libpq::Connection::new(dsn) {
            Ok(connection) => connection,
            Err(message) => {
                return Err(crate::Error::Connect {
                    dsn: dsn.to_string(),
                    message,
                })
            },
        };

        connection.set_error_verbosity(libpq::Verbosity::Terse);
        connection.set_client_encoding(libpq::Encoding::UTF8);

        unsafe {
            connection.set_notice_processor(
                Some(notice_processor),
                std::ptr::null_mut(),
            );
        }

        Ok(Self {
            connection: std::sync::Mutex::new(connection),
        })
    }

    pub fn r#async(&self) -> crate::Async<'_> {
        crate::Async::new(&self.connection)
    }

    /**
     * Creates a new connection from [`Config`].
     *
     * [`Config`]: struct.Config.html
     */
    pub fn from_config(config: &crate::Config) -> crate::Result<Self> {
        Self::new(&config.to_string())
    }

    pub fn model<'a, M>(&'a self) -> M
    where
        M: crate::Model<'a>,
    {
        M::new(self)
    }

    /**
     * Executes a simple text query, without parameter.
     */
    pub fn execute(&self, query: &str) -> crate::Result<crate::pq::Result> {
        self.connection.lock().unwrap().exec(&query).try_into()
    }

    /**
     * Executes a simple query, can have parameters.
     */
    pub fn query<E: crate::Entity>(
        &self,
        query: &str,
        params: &[&dyn crate::ToSql],
    ) -> crate::Result<crate::Rows<E>> {
        Ok(self.send_query(&query, params)?.into())
    }

    /**
     * Likes [`query`] but peaks only the first result.
     *
     * [`query`]: #method.query
     */
    pub fn query_one<E: crate::Entity>(
        &self,
        query: &str,
        params: &[&dyn crate::ToSql],
    ) -> crate::Result<E> {
        match self.query(&query, params)?.try_get(0) {
            Some(e) => Ok(e),
            None => Err(crate::Error::MissingField("0".to_string())),
        }
    }

    fn send_query(
        &self,
        query: &str,
        params: &[&dyn crate::ToSql],
    ) -> crate::Result<crate::pq::Result> {
        let mut param_types = Vec::new();
        let mut param_values = Vec::new();
        let mut param_formats = Vec::new();

        for param in params.iter() {
            param_types.push(param.ty().oid);
            param_values.push(param.to_sql()?);
            param_formats.push(param.format());
        }

        self.connection
            .lock()
            .unwrap()
            .exec_params(
                &self.order_parameters(query),
                &param_types,
                &param_values,
                &param_formats,
                crate::pq::Format::Binary,
            )
            .try_into()
    }

    fn order_parameters<'a>(
        &self,
        query: &'a str,
    ) -> std::borrow::Cow<'a, str> {
        lazy_static::lazy_static! {
            static ref REGEX: regex::Regex =
                #[allow(clippy::trivial_regex)]
                regex::Regex::new(r"\$\*").unwrap();
        }

        let mut count = 0;

        REGEX.replace_all(query, |captures: &regex::Captures<'_>| {
            count += 1;

            captures[0].replace("$*", &format!("${}", count))
        })
    }

    /**
     * Return an entity upon its primary key. If no entities are found, `None`
     * is returned.
     */
    pub fn find_by_pk<'a, M>(
        &self,
        pk: &HashMap<&str, &dyn crate::ToSql>,
    ) -> crate::Result<Option<M::Entity>>
    where
        M: crate::Model<'a>,
    {
        let (clause, params) = self.pk_clause::<M>(pk)?;
        let mut tuples = self.find_where::<M>(&clause, &params, None)?;

        Ok(match tuples.next() {
            Some(e) => Some(e),
            None => None,
        })
    }

    /**
     * Return all elements from a relation. If a suffix is given, it is append
     * to the query. This is mainly useful for "order by" statements.
     *
     * NOTE: suffix is inserted as is with NO ESCAPING. DO NOT use it to place
     * "where" condition nor any untrusted params.
     */
    pub fn find_all<'a, M>(
        &self,
        suffix: Option<&str>,
    ) -> crate::Result<crate::Rows<M::Entity>>
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

    /**
     * Perform a simple select on a given condition
     *
     * NOTE: suffix is inserted as is with NO ESCAPING. DO NOT use it to place
     * "where" condition nor any untrusted params.
     */
    pub fn find_where<'a, M>(
        &self,
        clause: &str,
        params: &[&dyn crate::ToSql],
        suffix: Option<&str>,
    ) -> crate::Result<crate::Rows<M::Entity>>
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

    /**
     * Paginate a query.
     *
     * This is done with limit/offset, read why it’s probably not a good idea to
     * use it: <https://use-the-index-luke.com/no-offset>.
     */
    pub fn paginate_find_where<'a, M>(
        &self,
        clause: &str,
        params: &[&dyn crate::ToSql],
        max_per_page: usize,
        page: usize,
        suffix: Option<&str>,
    ) -> crate::Result<crate::Pager<M::Entity>>
    where
        M: crate::Model<'a>,
    {
        let suffix = format!(
            "{} offset {} fetch first {} rows only",
            suffix.unwrap_or_default(),
            max_per_page * (page - 1),
            max_per_page
        );

        let rows = self.find_where::<M>(clause, params, Some(&suffix))?;
        let count = self.count_where::<M>(clause, params)?;

        let pager = crate::Pager::new(rows, count, page, max_per_page);

        Ok(pager)
    }

    /**
     * Return the number of records matching a condition.
     */
    pub fn count_where<'a, M>(
        &self,
        clause: &str,
        params: &[&dyn crate::ToSql],
    ) -> crate::Result<usize>
    where
        M: crate::Model<'a>,
    {
        let query = format!(
            "SELECT COUNT(*) FROM {} WHERE {};",
            M::Structure::relation(),
            clause,
        );

        let results = self.send_query(&query, params)?;

        Ok(results.get(0).try_get("count")?)
    }

    /**
     * Check if rows matching the given condition do exist or not.
     */
    pub fn exist_where<'a, M>(
        &self,
        clause: &str,
        params: &[&dyn crate::ToSql],
    ) -> crate::Result<bool>
    where
        M: crate::Model<'a>,
    {
        let query = format!(
            "SELECT EXISTS (SELECT true FROM {} WHERE {}) AS result;",
            M::Structure::relation(),
            clause,
        );

        let results = self.send_query(&query, params)?;

        Ok(results.get(0).try_get("result")?)
    }

    /**
     * Insert a new entity in the database.
     *
     * Returns the entity with values from database (ie: default values).
     */
    pub fn insert_one<'a, M>(
        &self,
        entity: &M::Entity,
    ) -> crate::Result<M::Entity>
    where
        M: crate::Model<'a>,
    {
        use crate::Entity;

        let mut tuple = Vec::new();
        let mut params = Vec::new();
        let mut fields = Vec::new();
        let mut x = 1;

        for field in M::Structure::columns() {
            if let Some(value) = entity.get(&field) {
                tuple.push(value);
                params.push(format!("${}", x));
                fields.push(*field);
                x += 1;
            }
        }

        let query = format!(
            "INSERT INTO {} ({}) VALUES({}) RETURNING {};",
            M::Structure::relation(),
            fields.join(", "),
            params.join(", "),
            M::create_projection(),
        );

        let results = self.send_query(&query, tuple.as_slice())?;

        Ok(M::create_entity(&results.get(0)))
    }

    /**
     * Update the entity.
     *
     * Returns the entity with values from database.
     */
    pub fn update_one<'a, M>(
        &self,
        pk: &HashMap<&str, &dyn crate::ToSql>,
        entity: &M::Entity,
    ) -> crate::Result<Option<M::Entity>>
    where
        M: crate::Model<'a>,
    {
        use crate::Entity;

        let mut data = HashMap::new();

        for field in M::Structure::columns() {
            let value = match entity.get(&field) {
                Some(value) => value,
                None => &Option::<&str>::None,
            };
            data.insert(field.to_string(), value);
        }

        self.update_by_pk::<M>(&pk, &data)
    }

    /**
     * Update a record and fetch it with its new values. If no records match
     * the given key, `None` is returned.
     */
    pub fn update_by_pk<'a, M>(
        &self,
        pk: &HashMap<&str, &dyn crate::ToSql>,
        data: &HashMap<String, &dyn crate::ToSql>,
    ) -> crate::Result<Option<M::Entity>>
    where
        M: crate::Model<'a>,
    {
        let (clause, mut params) = self.pk_clause::<M>(&pk)?;
        let mut x = params.len() + 1;
        let mut set = Vec::new();
        let projection = M::default_projection();

        for (key, value) in data.iter() {
            if projection.has_field(key) {
                set.push(format!("{} = ${}", key, x));
                params.push(*value);
                x += 1;
            }
        }

        let query = format!(
            "UPDATE {} SET {} WHERE {} RETURNING {};",
            M::Structure::relation(),
            set.join(", "),
            clause,
            M::create_projection(),
        );

        let results = self.send_query(&query, &params)?;

        let entity = results.try_get(0).map(|x| M::create_entity(&x));

        Ok(entity)
    }

    /**
     * Delete an entity from a table.
     *
     * Returns the entity fetched from the deleted record.
     */
    pub fn delete_one<'a, M>(
        &self,
        entity: &M::Entity,
    ) -> crate::Result<Option<M::Entity>>
    where
        M: crate::Model<'a>,
    {
        let pk = M::primary_key(&entity);

        self.delete_by_pk::<M>(&pk)
    }

    /**
     * Delete a record from its primary key. The deleted entity is returned or
     * `None` if not found.
     */
    pub fn delete_by_pk<'a, M>(
        &self,
        pk: &HashMap<&str, &dyn crate::ToSql>,
    ) -> crate::Result<Option<M::Entity>>
    where
        M: crate::Model<'a>,
    {
        let (clause, params) = self.pk_clause::<M>(&pk)?;
        let mut results = self.delete_where::<M>(&clause, &params)?;

        Ok(results.next())
    }

    /**
     * Delete records by a given condition. A collection of all deleted entries
     * is returned.
     */
    pub fn delete_where<'a, M>(
        &self,
        clause: &str,
        params: &[&dyn crate::ToSql],
    ) -> crate::Result<crate::Rows<M::Entity>>
    where
        M: crate::Model<'a>,
    {
        let query = format!(
            "DELETE FROM {} WHERE {} RETURNING {};",
            M::Structure::relation(),
            clause,
            M::create_projection(),
        );

        self.query(&query, &params)
    }

    fn pk_clause<'a, 'b, M>(
        &self,
        pk: &HashMap<&str, &'b dyn crate::ToSql>,
    ) -> crate::Result<(String, Vec<&'b dyn crate::ToSql>)>
    where
        M: crate::Model<'a>,
    {
        let keys: Vec<_> = pk.keys().copied().collect();

        if keys != M::Structure::primary_key() {
            return Err(crate::Error::PrimaryKey);
        }

        let clause =
            keys.iter().enumerate().fold(String::new(), |acc, (i, x)| {
                let field = format!("\"{}\"", x.replace("\"", "\\\""));

                if acc.is_empty() {
                    format!("{} = ${}", field, i + 1)
                }
                else {
                    format!("{} AND {} = ${}", acc, field, i + 1)
                }
            });

        let params: Vec<_> = pk.values().copied().collect();

        Ok((clause, params))
    }

    /**
     * Determines if the connection is no longer usable.
     */
    pub fn has_broken(&self) -> bool {
        self.connection.lock().unwrap().status()
            == libpq::connection::Status::Bad
    }
}
