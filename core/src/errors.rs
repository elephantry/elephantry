pub type Result<T> = std::result::Result<T, crate::Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /** An error in async context. */
    #[error("Async error: {0}")]
    Async(String),
    /** Configuration error */
    #[cfg(feature = "config-support")]
    #[error("Config error: {0}")]
    Config(#[from] config::ConfigError),
    /** Connection error */
    #[error("{message}")]
    Connect { dsn: String, message: String },
    /** Escaping error */
    #[error("Unable to escape '{0}': {1}")]
    Escape(String, String),
    /** Unable to transform a SQL field in rust value */
    #[error("Invalid {rust_type} value: {value}")]
    FromSql {
        pg_type: crate::pq::Type,
        rust_type: String,
        value: String,
    },
    /** Inspector error */
    #[cfg(feature = "v2")]
    #[error("{0}")]
    Inspect(String),
    /** Input/Output error */
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /** Our result set require an extra field to build the entity */
    #[error("Missing field {0}")]
    MissingField(String),
    /** Fetch a null value in a non-option type */
    #[error("Try to retreive null field as non-option type")]
    NotNull,
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
    #[error("Invalid utf8 value: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    /** XML error */
    #[cfg(feature = "xml")]
    #[error("Xml error: {0}")]
    Xml(#[from] xmltree::Error),
}
