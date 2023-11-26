use crate::Projectable;
use crate::Structure;
use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct Param {
    pub types: Vec<u32>,
    pub values: Vec<Option<Vec<u8>>>,
    pub formats: Vec<crate::pq::Format>,
}

/**
 * Result type of [`ping`] function.
 *
 * [`ping`]: crate::Connection::ping
 */
pub type PingStatus = libpq::ping::Status;

/**
 * A connection to a database.
 */
#[derive(Clone, Debug)]
pub struct Connection {
    pub(crate) connection: std::sync::Arc<std::sync::Mutex<libpq::Connection>>,
}

extern "C" fn notice_processor(_arg: *mut std::ffi::c_void, message: *const std::ffi::c_char) {
    let message = unsafe { std::ffi::CStr::from_ptr(message) };

    log::info!("{}", message.to_str().unwrap().trim());
}

impl Connection {
    pub fn new(dsn: &str) -> crate::Result<Self> {
        let connection = match libpq::Connection::new(dsn) {
            Ok(connection) => connection,
            Err(error) => {
                return Err(crate::Error::Connect {
                    dsn: dsn.to_string(),
                    error,
                })
            }
        };

        connection.set_error_verbosity(libpq::Verbosity::Terse);
        connection.set_client_encoding(libpq::Encoding::UTF8);

        unsafe {
            connection.set_notice_processor(Some(notice_processor), std::ptr::null_mut());
        }

        Ok(Self {
            connection: std::sync::Arc::new(std::sync::Mutex::new(connection)),
        })
    }

    #[must_use]
    pub fn r#async(&self) -> crate::Async<'_> {
        crate::Async::new(&self.connection)
    }

    #[must_use]
    pub fn transaction(&self) -> crate::Transaction<'_> {
        crate::Transaction::new(self)
    }

    pub(crate) fn transaction_status(&self) -> crate::Result<libpq::transaction::Status> {
        let status = self
            .connection
            .lock()
            .map_err(|e| crate::Error::Mutex(e.to_string()))?
            .transaction_status();

        Ok(status)
    }

    pub(crate) fn escape_identifier(&self, str: &str) -> crate::Result<String> {
        self.connection
            .lock()
            .map_err(|e| crate::Error::Mutex(e.to_string()))?
            .escape_identifier(str)
            .map_err(|e| crate::Error::Escape(str.to_string(), e))
            .map(|x| String::from_utf8_lossy(x.as_ref()).to_string())
    }

    /**
     * Creates a new connection from [`Config`].
     *
     * [`Config`]: crate::Config
     */
    pub fn from_config(config: &crate::Config) -> crate::Result<Self> {
        Self::new(&config.to_string())
    }

    #[must_use]
    pub fn model<M>(&self) -> M
    where
        M: crate::Model,
    {
        M::new(self)
    }

    /**
     * Executes a simple text query, without parameter.
     */
    pub fn execute(&self, query: &str) -> crate::Result<crate::pq::Result> {
        self.connection
            .lock()
            .map_err(|e| crate::Error::Mutex(e.to_string()))?
            .exec(query)
            .try_into()
    }

    /**
     * Executes a simple query, can have parameters.
     */
    pub fn query<E: crate::Entity>(
        &self,
        query: &str,
        params: &[&dyn crate::ToSql],
    ) -> crate::Result<crate::Rows<E>> {
        Ok(self.send_query(query, params)?.into())
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
        match self.query(query, params)?.try_get(0) {
            Some(e) => Ok(e),
            None => Err(crate::Error::MissingField("0".to_string())),
        }
    }

    fn send_query(
        &self,
        query: &str,
        params: &[&dyn crate::ToSql],
    ) -> crate::Result<crate::pq::Result> {
        let param = Self::transform_params(params)?;

        self.connection
            .lock()
            .map_err(|e| crate::Error::Mutex(e.to_string()))?
            .exec_params(
                &Self::order_parameters(query),
                &param.types,
                &param.values,
                &param.formats,
                crate::pq::Format::Binary,
            )
            .try_into()
    }

    pub(crate) fn transform_params(params: &[&dyn crate::ToSql]) -> crate::Result<Param> {
        let mut p = Param::default();

        for param in params {
            p.types.push(param.ty().oid);
            p.values.push(param.to_text()?.map(|mut x| {
                x.push('\0');
                x.into_bytes()
            }));
            p.formats.push(crate::pq::Format::Text);
        }

        Ok(p)
    }

    fn order_parameters(query: &str) -> std::borrow::Cow<'_, str> {
        let regex = crate::regex!(r"\$\*");

        let mut count = 0;

        regex.replace_all(query, |captures: &regex::Captures<'_>| {
            count += 1;

            captures[0].replace("$*", &format!("${count}"))
        })
    }

    /**
     * Return an entity upon its primary key. If no entities are found, `None`
     * is returned.
     */
    pub fn find_by_pk<M>(
        &self,
        pk: &HashMap<&str, &dyn crate::ToSql>,
    ) -> crate::Result<Option<M::Entity>>
    where
        M: crate::Model,
    {
        let (clause, params) = Self::pk_clause::<M>(pk)?;
        let mut tuples = self.find_where::<M>(&clause, &params, None)?;

        Ok(tuples.next())
    }

    /**
     * Return all elements from a relation. If a suffix is given, it is append
     * to the query. This is mainly useful for "order by" statements.
     *
     * NOTE: suffix is inserted as is with NO ESCAPING. DO NOT use it to place
     * "where" condition nor any untrusted params.
     */
    pub fn find_all<M>(&self, suffix: Option<&str>) -> crate::Result<crate::Rows<M::Entity>>
    where
        M: crate::Model,
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
    pub fn find_where<M>(
        &self,
        clause: &str,
        params: &[&dyn crate::ToSql],
        suffix: Option<&str>,
    ) -> crate::Result<crate::Rows<M::Entity>>
    where
        M: crate::Model,
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
    pub fn paginate_find_where<M>(
        &self,
        clause: &str,
        params: &[&dyn crate::ToSql],
        max_per_page: usize,
        page: usize,
        suffix: Option<&str>,
    ) -> crate::Result<crate::Pager<M::Entity>>
    where
        M: crate::Model,
    {
        let suffix = format!(
            "{} offset {} fetch first {max_per_page} rows only",
            suffix.unwrap_or_default(),
            max_per_page * (page - 1),
        );

        let rows = self.find_where::<M>(clause, params, Some(&suffix))?;
        let count = self.count_where::<M>(clause, params)?;

        let pager = crate::Pager::new(rows, count, page, max_per_page);

        Ok(pager)
    }

    /**
     * Return the number of records matching a condition.
     */
    pub fn count_where<M>(&self, clause: &str, params: &[&dyn crate::ToSql]) -> crate::Result<usize>
    where
        M: crate::Model,
    {
        let query = format!(
            "SELECT COUNT(*) FROM {} WHERE {clause};",
            M::Structure::relation(),
        );

        let results = self.send_query(&query, params)?;

        results.get(0).try_get("count")
    }

    /**
     * Check if rows matching the given condition do exist or not.
     */
    pub fn exist_where<M>(&self, clause: &str, params: &[&dyn crate::ToSql]) -> crate::Result<bool>
    where
        M: crate::Model,
    {
        let query = format!(
            "SELECT EXISTS (SELECT true FROM {} WHERE {clause}) AS result;",
            M::Structure::relation(),
        );

        let results = self.send_query(&query, params)?;

        results.get(0).try_get("result")
    }

    /**
     * Insert a new entity in the database.
     *
     * Returns the entity with values from database (ie: default values).
     */
    pub fn insert_one<M>(&self, entity: &M::Entity) -> crate::Result<M::Entity>
    where
        M: crate::Model,
    {
        self.insert::<M>(entity, None).map(Option::unwrap)
    }

    /**
     * Try to insert a new entity in the database. On constraint violation error
     * on `target` you can do an alternative action `action`.
     *
     * See [ON CONFLICT clause](https://www.postgresql.org/docs/current/sql-insert.html#SQL-ON-CONFLICT).
     *
     * Returns the entity with values from database (ie: default values).
     */
    pub fn upsert_one<M>(
        &self,
        entity: &M::Entity,
        target: &str,
        action: &str,
    ) -> crate::Result<Option<M::Entity>>
    where
        M: crate::Model,
    {
        let suffix = format!("on conflict {target} do {action}");
        self.insert::<M>(entity, Some(suffix.as_str()))
    }

    fn insert<M>(
        &self,
        entity: &M::Entity,
        suffix: Option<&str>,
    ) -> crate::Result<Option<M::Entity>>
    where
        M: crate::Model,
    {
        use crate::Entity;

        let mut tuple = Vec::new();
        let mut params = Vec::new();
        let mut fields = Vec::new();
        let mut x = 1;

        for field in M::Structure::columns() {
            if let Some(value) = entity.get(field) {
                tuple.push(value);
                params.push(format!("${x}"));
                fields.push(*field);
                x += 1;
            }
        }

        let query = format!(
            "INSERT INTO {} ({}) VALUES({}) {} RETURNING {};",
            M::Structure::relation(),
            fields.join(", "),
            params.join(", "),
            suffix.unwrap_or_default(),
            M::create_projection(),
        );

        let results = self.send_query(&query, tuple.as_slice())?;
        let result = results.try_get(0).map(|x| M::create_entity(&x));

        Ok(result)
    }

    /**
     * Update the entity.
     *
     * Returns the entity with values from database.
     */
    pub fn update_one<M>(
        &self,
        pk: &HashMap<&str, &dyn crate::ToSql>,
        entity: &M::Entity,
    ) -> crate::Result<Option<M::Entity>>
    where
        M: crate::Model,
    {
        use crate::Entity;

        let mut data = HashMap::new();

        for field in M::Structure::columns() {
            let value = match entity.get(field) {
                Some(value) => value,
                None => &Option::<&str>::None,
            };
            data.insert((*field).to_string(), value);
        }

        self.update_by_pk::<M>(pk, &data)
    }

    /**
     * Update a record and fetch it with its new values. If no records match
     * the given key, `None` is returned.
     */
    pub fn update_by_pk<M>(
        &self,
        pk: &HashMap<&str, &dyn crate::ToSql>,
        data: &HashMap<String, &dyn crate::ToSql>,
    ) -> crate::Result<Option<M::Entity>>
    where
        M: crate::Model,
    {
        let (clause, mut params) = Self::pk_clause::<M>(pk)?;
        let mut x = params.len() + 1;
        let mut set = Vec::new();
        let projection = M::default_projection();

        for (key, value) in data {
            if projection.has_field(key) {
                set.push(format!("{key} = ${x}"));
                params.push(*value);
                x += 1;
            }
        }

        if set.is_empty() {
            log::warn!("No field to update");
            return Ok(None);
        }

        let query = format!(
            "UPDATE {} SET {} WHERE {clause} RETURNING {};",
            M::Structure::relation(),
            set.join(", "),
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
    pub fn delete_one<M>(&self, entity: &M::Entity) -> crate::Result<Option<M::Entity>>
    where
        M: crate::Model,
    {
        let pk = M::primary_key(entity)?;

        self.delete_by_pk::<M>(&pk)
    }

    /**
     * Delete a record from its primary key. The deleted entity is returned or
     * `None` if not found.
     */
    pub fn delete_by_pk<M>(
        &self,
        pk: &HashMap<&str, &dyn crate::ToSql>,
    ) -> crate::Result<Option<M::Entity>>
    where
        M: crate::Model,
    {
        let (clause, params) = Self::pk_clause::<M>(pk)?;
        let mut results = self.delete_where::<M>(&clause, &params)?;

        Ok(results.next())
    }

    /**
     * Delete records by a given condition. A collection of all deleted entries
     * is returned.
     */
    pub fn delete_where<M>(
        &self,
        clause: &str,
        params: &[&dyn crate::ToSql],
    ) -> crate::Result<crate::Rows<M::Entity>>
    where
        M: crate::Model,
    {
        let query = format!(
            "DELETE FROM {} WHERE {clause} RETURNING {};",
            M::Structure::relation(),
            M::create_projection(),
        );

        self.query(&query, params)
    }

    fn pk_clause<'a, M>(
        pk: &HashMap<&str, &'a dyn crate::ToSql>,
    ) -> crate::Result<(String, Vec<&'a dyn crate::ToSql>)>
    where
        M: crate::Model,
    {
        let keys: Vec<_> = pk.keys().copied().collect();

        if keys != M::Structure::primary_key() {
            return Err(crate::Error::PrimaryKey);
        }

        let clause = keys.iter().enumerate().fold(String::new(), |acc, (i, x)| {
            let field = format!("\"{}\"", x.replace('"', "\\\""));

            if acc.is_empty() {
                format!("{field} = ${}", i + 1)
            } else {
                format!("{acc} AND {field} = ${}", i + 1)
            }
        });

        let params: Vec<_> = pk.values().copied().collect();

        Ok((clause, params))
    }

    /**
     * Determines if the connection is no longer usable.
     */
    pub fn has_broken(&self) -> crate::Result<bool> {
        let status = self
            .connection
            .lock()
            .map_err(|e| crate::Error::Mutex(e.to_string()))?
            .status();

        Ok(status == libpq::connection::Status::Bad)
    }

    /**
     * Send a NOTIFY event to the database server. An optional data can be sent
     * with the notification.
     */
    pub fn notify(&self, channel: &str, data: Option<&str>) -> crate::Result {
        let data = self.escape_literal(data.unwrap_or_default())?;

        let query = format!("notify {channel}, {data}");

        self.execute(&query).map(|_| ())
    }

    /**
     * Start to listen on the given channel.
     *
     * Note: when listen is issued in a transaction it is unlisten when the
     * transaction is committed or rollback.
     */
    pub fn listen(&self, channel: &str) -> crate::Result {
        let query = format!("listen {channel}");

        self.execute(&query).map(|_| ())
    }

    /**
     * Stop to listen on the given channel.
     */
    pub fn unlisten(&self, channel: &str) -> crate::Result {
        let query = format!("unlisten {channel}");

        self.execute(&query).map(|_| ())
    }

    /**
     * Check if a notification is pending. If so, the payload is returned.
     * Otherwise, `None` is returned.
     */
    pub fn notifies(&self) -> crate::Result<Option<crate::pq::Notify>> {
        let connection = self
            .connection
            .lock()
            .map_err(|e| crate::Error::Mutex(e.to_string()))?;

        connection.consume_input().ok();
        Ok(connection.notifies())
    }

    fn escape_literal(&self, str: &str) -> crate::Result<String> {
        self.connection
            .lock()
            .map_err(|e| crate::Error::Mutex(e.to_string()))?
            .escape_literal(str)
            .map_err(|e| crate::Error::Escape(str.to_string(), e))
            .map(|x| String::from_utf8_lossy(x.as_ref()).to_string())
    }

    /**
     * Reports the status of the server.
     */
    pub fn ping(&self) -> crate::Result {
        let connection = self
            .connection
            .lock()
            .map_err(|e| crate::Error::Mutex(e.to_string()))?;

        let mut params = HashMap::new();
        params.insert("dbname".to_string(), connection.db()?);
        params.insert("host".to_string(), connection.host()?);
        params.insert("port".to_string(), connection.port()?);
        params.insert("user".to_string(), connection.user()?);
        if let Some(password) = connection.pass()? {
            params.insert("password".to_string(), password);
        }

        match libpq::Connection::ping_params(&params, false) {
            PingStatus::Ok => Ok(()),
            status => Err(crate::Error::Ping(status)),
        }
    }

    /**
     * Retreives connection configuration.
     */
    pub fn config(&self) -> crate::Result<crate::Config> {
        let connection = self
            .connection
            .lock()
            .map_err(|e| crate::Error::Mutex(e.to_string()))?;
        let info = connection.info()?;

        let config = crate::Config {
            application_name: info.get("application_name").and_then(|x| x.val.clone()),
            channel_binding: Self::config_get(&info, "channel_binding")?,
            client_encoding: info.get("client_encoding").and_then(|x| x.val.clone()),
            connect_timeout: Self::config_get(&info, "connect_timeout")?,
            dbname: info.get("dbname").and_then(|x| x.val.clone()),
            fallback_application_name: info
                .get("fallback_application_name")
                .and_then(|x| x.val.clone()),
            gssencmode: Self::config_get(&info, "gssencmode")?,
            gsslib: info.get("gsslib").and_then(|x| x.val.clone()),
            hostaddr: info.get("hostaddr").and_then(|x| x.val.clone()),
            host: info.get("host").and_then(|x| x.val.clone()),
            keepalives_count: Self::config_get(&info, "keepalives_count")?,
            keepalives_idle: Self::config_get(&info, "keepalives_idle")?,
            keepalives_interval: Self::config_get(&info, "keepalives_interval")?,
            keepalives: Self::config_get::<i32>(&info, "keepalives")?.map(|x| x == 1),
            krbsrvname: info.get("krbsrvname").and_then(|x| x.val.clone()),
            options: info.get("options").and_then(|x| x.val.clone()),
            passfile: info.get("passfile").and_then(|x| x.val.clone()),
            password: info.get("password").and_then(|x| x.val.clone()),
            port: info.get("port").and_then(|x| x.val.clone()),
            replication: info.get("replication").and_then(|x| x.val.clone()),
            requirepeer: info.get("requirepeer").and_then(|x| x.val.clone()),
            service: info.get("service").and_then(|x| x.val.clone()),
            sslcert: info.get("sslcert").and_then(|x| x.val.clone()),
            sslcompression: Self::config_get::<i32>(&info, "sslcompression")?.map(|x| x == 1),
            sslcrl: info.get("sslcrl").and_then(|x| x.val.clone()),
            sslkey: info.get("sslkey").and_then(|x| x.val.clone()),
            ssl_max_protocol_version: info
                .get("ssl_max_protocol_version")
                .and_then(|x| x.val.clone()),
            ssl_min_protocol_version: info
                .get("ssl_min_protocol_version")
                .and_then(|x| x.val.clone()),
            sslmode: Self::config_get(&info, "sslmode")?,
            sslpassword: info.get("sslpassword").and_then(|x| x.val.clone()),
            sslrootcert: info.get("sslrootcert").and_then(|x| x.val.clone()),
            target_session_attrs: Self::config_get(&info, "target_session_attrs")?,
            tcp_user_timeout: Self::config_get(&info, "tcp_user_timeout")?,
            user: info.get("user").and_then(|x| x.val.clone()),
        };

        Ok(config)
    }

    fn config_get<T>(
        info: &HashMap<String, libpq::connection::Info>,
        name: &str,
    ) -> Result<Option<T>, <T as std::str::FromStr>::Err>
    where
        T: std::str::FromStr,
    {
        let r = match info.get(name).map(|x| x.val.clone()) {
            Some(Some(val)) => Some(val.parse()?),
            _ => None,
        };

        Ok(r)
    }

    /**
     * Bulk insert entities via COPY mode.
     */
    pub fn copy<M, I>(&self, entities: I) -> crate::Result
    where
        I: Iterator<Item = M::Entity>,
        M: crate::Model,
    {
        use crate::Entity;

        let projection = M::default_projection();
        let field_names = projection.field_names();

        let query = format!(
            "copy {} ({}) from stdin (format binary);",
            M::Structure::relation(),
            field_names.join(", "),
        );
        self.execute(&query)?;

        let connection = self
            .connection
            .lock()
            .map_err(|e| crate::Error::Mutex(e.to_string()))?;

        // Signature
        let mut buf = vec![
            b'P', b'G', b'C', b'O', b'P', b'Y', b'\n', 255, b'\r', b'\n', b'\0',
        ];
        // Flags field
        crate::to_sql::write_i32(&mut buf, 0)?;
        // Header extension area length
        crate::to_sql::write_i32(&mut buf, 0)?;

        for entity in entities {
            crate::to_sql::write_i16(&mut buf, field_names.len() as i16)?;

            for field in &field_names {
                let value = match entity.get(field) {
                    Some(value) => value.to_binary()?,
                    None => None,
                };

                if let Some(mut value) = value {
                    crate::to_sql::write_i32(&mut buf, value.len() as i32)?;
                    buf.append(&mut value);
                } else {
                    crate::to_sql::write_i32(&mut buf, -1)?;
                }
            }
        }
        crate::to_sql::write_i16(&mut buf, -1)?;

        connection.put_copy_data(&buf).map_err(crate::Error::Copy)?;

        connection.put_copy_end(None).map_err(crate::Error::Copy)?;

        if let Some(result) = connection.result() {
            if result.status() == libpq::Status::FatalError {
                return Err(crate::Error::Copy(libpq::errors::Error::Backend(
                    result.error_message()?.unwrap_or_default(),
                )));
            }
        }

        Ok(())
    }
}
