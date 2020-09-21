pub type Result<T> = std::result::Result<T, crate::Error>;

#[derive(Debug)]
pub enum Error {
    /** An error in async context. */
    Async(String),
    /** Connection error */
    Connect { dsn: String, message: String },
    /** Unable to transform a SQL field in rust value */
    FromSql {
        pg_type: crate::pq::Type,
        rust_type: String,
        value: String,
    },
    /** Input/Output error */
    Io(std::io::Error),
    /** Our result set require an extra field to build the entity */
    MissingField(String),
    /** Fetch a null value in a non-option type */
    NotNull,
    /** Incomplete primary key */
    PrimaryKey,
    /** SQL error */
    Sql(crate::pq::Result),
    /** Unable to transform a rust value to SQL */
    ToSql {
        pg_type: crate::pq::Type,
        rust_type: String,
        message: Option<String>,
    },
    /** UTF8 error */
    Utf8(std::string::FromUtf8Error),
    /** XML error */
    #[cfg(feature = "xml")]
    Xml(xmltree::Error),
}

impl std::error::Error for Error {
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::Async(message) => format!("Async error: {}", message),
            Error::Connect {
                message, ..
            } => message.clone(),
            Error::Sql(result) => {
                result
                    .error_message()
                    .unwrap_or_else(|| "Unknow SQL error".to_string())
            },
            Error::MissingField(field) => format!("Missing field {}", field),
            Error::NotNull => {
                "Try to retreive null field as non-option type".to_string()
            },
            Error::Io(err) => format!("I/O error: {}", err),
            Error::FromSql {
                rust_type,
                value,
                ..
            } => format!("Invalid {} value: {}", rust_type, value),
            Error::PrimaryKey => "Invalid primary key".to_string(),
            Error::ToSql {
                rust_type,
                message,
                ..
            } => {
                format!(
                    "Invalid {} value: '{}'",
                    rust_type,
                    message.clone().unwrap_or_else(|| "unknow".to_string())
                )
            },
            Error::Utf8(err) => format!("Invalid utf8 value: {}", err),
            #[cfg(feature = "xml")]
            Error::Xml(err) => format!("Xml error: {}", err),
        };

        write!(f, "{}", s)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::Utf8(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

#[cfg(feature = "xml")]
impl From<xmltree::Error> for Error {
    fn from(err: xmltree::Error) -> Self {
        Error::Xml(err)
    }
}
