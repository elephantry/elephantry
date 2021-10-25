pub type Result<T = ()> = std::result::Result<T, crate::Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /** An error in async context. */
    #[error("Async error: {0}")]
    Async(String),
    /** Configuration error */
    #[cfg(feature = "config-support")]
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    /** Connection error */
    #[error("{message}")]
    Connect { dsn: String, message: String },
    /** Copy error */
    #[error("Copy error: {0}")]
    Copy(String),
    /** Escaping error */
    #[error("Unable to escape '{0}': {1}")]
    Escape(String, String),
    /** Unable to transform a SQL field in rust value */
    #[error("Unable to convert from SQL {} (oid={}) to {rust_type}: {value}. Try {}", pg_type.name, pg_type.oid, crate::pq::sql_to_rust(pg_type))]
    FromSql {
        pg_type: crate::pq::Type,
        rust_type: String,
        value: String,
    },
    /** Inspector error */
    #[error("{0}")]
    Inspect(String),
    /** Input/Output error */
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /** Our result set require an extra field to build the entity */
    #[error("Missing field {0}")]
    MissingField(String),
    /** Connection mutex poisoned */
    #[error("Mutex error: {0}")]
    Mutex(String),
    /** Fetch a null value in a non-option type */
    #[error("Try to retreive null field as non-option type")]
    NotNull,
    /** Parse error */
    #[error("{0}")]
    Parse(String),
    /** Parse bool error */
    #[error(transparent)]
    ParseBoolError(#[from] std::str::ParseBoolError),
    /** Parse int error */
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    /** Ping error */
    #[error("Ping error: {0:?}")]
    Ping(crate::connection::PingStatus),
    /** Incomplete primary key */
    #[error("Invalid primary key")]
    PrimaryKey,
    /** SQL error */
    #[error("{}", .0.error_message().unwrap_or_else(|| "Unknow SQL error".to_string()))]
    Sql(crate::pq::Result),
    /** Unable to transform a rust value to SQL */
    #[error("Invalid {rust_type} value: '{}'", message.clone().unwrap_or_else(|| "unknow".to_string()))]
    ToSql {
        pg_type: crate::pq::Type,
        rust_type: String,
        message: Option<String>,
    },
    /** UTF8 error */
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
    /** XML error */
    #[cfg(feature = "xml")]
    #[error(transparent)]
    Xml(#[from] xmltree::Error),
}
