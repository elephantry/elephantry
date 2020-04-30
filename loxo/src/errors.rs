pub type Result<T> = std::result::Result<T, crate::Error>;

#[derive(Debug)]
pub enum Error {
    Connect {
        dsn: String,
        message: String,
    },
    FromSql {
        pg_type: crate::pq::Type,
        rust_type: String,
        value: String,
    },
    Io(std::io::Error),
    MissingField(String),
    Sql(crate::pq::Result),
    ToSql {
        pg_type: crate::pq::Type,
        rust_type: String,
        message: Option<String>,
    },
    Utf8(std::string::FromUtf8Error),
}

impl std::error::Error for Error {
}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let s = match self {
            Error::Connect { message, .. } => message.clone(),
            Error::Sql(result) => result.error_message().unwrap_or_else(|| "Unknow SQL error".to_string()),
            Error::MissingField(field) => format!("Missing field {}", field),
            Error::Io(err) => format!("I/O error: {}", err),
            Error::FromSql { rust_type, value, .. } => format!("Invalid {} value: {}", rust_type, value),
            Error::ToSql { rust_type, message, .. } => format!("Invalid {} value: '{}'", rust_type, message.clone().unwrap_or_else(|| "unknow".to_string())),
            Error::Utf8(err) => format!("Invalid utf8 value: {}", err),
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
