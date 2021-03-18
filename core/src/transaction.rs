/**
 * This `struct` is created by the [`Connection::transaction`] method.
 *
 * [`Connection::transaction`]: crate::Connection::transaction
 */
pub struct Transaction<'c> {
    pub(crate) connection: &'c crate::Connection,
}

/**
 * <http://www.postgresql.org/docs/current/sql-set-constraints.html>
 */
pub enum Constraints {
    Deferred,
    Immediate,
}

impl std::fmt::Display for Constraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Deferred => "deferred",
            Self::Immediate => "immediate",
        };

        f.write_str(s)
    }
}

/**
 * <https://www.postgresql.org/docs/current/sql-set-transaction.html>
 */
pub enum IsolationLevel {
    /**
     * A statement can only see rows committed before it began. This is the
     * default.
     */
    ReadCommitted,
    /**
     * All statements of the current transaction can only see rows committed
     * before the first query or data-modification statement was executed in
     * this transaction.
     */
    RepeatableRead,
    /**
     * All statements of the current transaction can only see rows committed
     * before the first query or data-modification statement was executed in
     * this transaction. If a pattern of reads and writes among concurrent
     * serializable transactions would create a situation which could not have
     * occurred for any serial (one-at-a-time) execution of those transactions,
     * one of them will be rolled back with a serialization_failure error.
     */
    Serializable,
}

impl std::fmt::Display for IsolationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::ReadCommitted => "read committed",
            Self::RepeatableRead => "repeatable read",
            Self::Serializable => "serializable",
        };

        f.write_str(s)
    }
}

/**
 * <https://www.postgresql.org/docs/current/sql-set-transaction.html>
 */
pub enum AccessMode {
    ReadOnly,
    ReadWrite,
}

impl std::fmt::Display for AccessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::ReadOnly => "read only",
            Self::ReadWrite => "read write",
        };

        f.write_str(s)
    }
}

impl<'c> Transaction<'c> {
    pub(crate) fn new(connection: &'c crate::Connection) -> Self {
        Self {
            connection,
        }
    }

    /**
     * Start a new transaction.
     */
    pub fn start(&self) -> crate::Result {
        self.exec("begin transaction")
    }

    /**
     * Commit a transaction.
     */
    pub fn commit(&self) -> crate::Result {
        self.exec("commit transaction")
    }

    /**
     * Rollback a transaction. If a `name` is specified, the transaction is
     * rollback to the given savepoint. Otherwise, the whole transaction is
     * rollback.
     */
    pub fn roolback(&self, name: Option<&str>) -> crate::Result {
        let query = match name {
            Some(name) => format!("rollback to savepoint {}", name),
            None => "rollback transaction".to_string(),
        };

        self.exec(&query)
    }

    /**
     * Set a savepoint in a transaction.
     */
    pub fn set_save_point(&self, name: &str) -> crate::Result {
        let query = format!("savepoint {}", name);

        self.exec(&query)
    }

    /**
     * Drop a savepoint.
     */
    pub fn release_savepoint(&self, name: &str) -> crate::Result {
        let query = format!("release savepoint {}", name);

        self.exec(&query)
    }

    /**
     * Tell if a transaction is open or not.
     */
    #[deprecated(note = "use v2::transaction::is_in_transaction instead", since = "1.7.0")]
    pub fn is_in_transaction(&self) -> bool {
        crate::v2::transaction::is_in_transaction(self).unwrap()
    }

    /**
     * In PostgreSQL, an error during a transaction cancels all the queries and
     * rollback the transaction on commit. This method returns the current
     * transaction's status. If no transactions are open, it returns `None`.
     */
    #[deprecated(note = "use v2::transaction::is_transaction_ok instead", since = "1.7.0")]
    pub fn is_transaction_ok(&self) -> Option<bool> {
        crate::v2::transaction::is_transaction_ok(self).unwrap()
    }

    /**
     * Set given constraints to deferred/immediate in the current transaction.
     * This applies to constraints being deferrable or deferred by default.
     * If the keys is `None`, ALL keys will be set at the given state.
     *
     * See <http://www.postgresql.org/docs/current/sql-set-constraints.html>
     */
    pub fn set_deferrable(
        &self,
        keys: Option<Vec<&str>>,
        constraints: Constraints,
    ) -> crate::Result {
        let name = if let Some(keys) = keys {
            keys.iter()
                .map(|key| self.escape_identifier(key))
                .collect::<crate::Result<Vec<_>>>()?
                .join(", ")
        }
        else {
            "ALL".to_string()
        };

        let query = format!("set constraints {} {}", name, constraints);

        self.exec(&query)
    }

    fn escape_identifier(&self, id: &str) -> crate::Result<String> {
        id.split('.')
            .map(|x| self.connection.escape_identifier(x))
            .collect::<crate::Result<Vec<_>>>()
            .map(|x| x.join("."))
    }

    /**
     * Transaction isolation level tells PostgreSQL how to manage with the
     * current transaction. The default is "READ COMMITTED".
     *
     * See <http://www.postgresql.org/docs/current/sql-set-transaction.html>
     */
    pub fn set_isolation_level(
        &self,
        level: IsolationLevel,
    ) -> crate::Result {
        let query = format!("set transaction isolation level {}", level);

        self.exec(&query)
    }

    /**
     * Transaction access modes tell PostgreSQL if transaction are able to
     * write or read only.
     *
     * See <http://www.postgresql.org/docs/current/sql-set-transaction.html>
     */
    pub fn set_access_mode(&self, mode: AccessMode) -> crate::Result {
        let query = format!("set transaction {}", mode);

        self.exec(&query)
    }

    fn exec(&self, query: &str) -> crate::Result {
        self.connection.execute(query).map(|_| ())
    }
}
