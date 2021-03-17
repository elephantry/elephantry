/**
 * Connection configuration.
 */
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Config {
    pub dbname: Option<String>,
    pub host: Option<String>,
    pub password: Option<String>,
    pub port: Option<String>,
    pub user: Option<String>,
}

macro_rules! get {
    ($config:ident . $field:ident, $env:expr, $default:expr) => {
        $config
            .$field
            .clone()
            .or(std::env::var($env).ok())
            .unwrap_or_else(|| $default)
    };
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    #[deprecated(note="Use Connection::config() instead", since="1.7.0")]
    pub fn user(&self) -> String {
        get!(self.user, "PGUSER", std::env::var("USER").unwrap())
    }

    #[deprecated(note="Use Connection::config() instead", since="1.7.0")]
    pub fn host(&self) -> String {
        get!(self.host, "PGHOST", "/run/postgresql".to_string())
    }

    #[deprecated(note="Use Connection::config() instead", since="1.7.0")]
    pub fn dbname(&self) -> String {
        #![allow(deprecated)]
        get!(self.dbname, "PGDATABASE", self.user())
    }

    #[deprecated(note="Use Connection::config() instead", since="1.7.0")]
    pub fn port(&self) -> String {
        get!(self.port, "PGPORT", "5432".to_string())
    }

    #[deprecated(note="Use Connection::config() instead", since="1.7.0")]
    pub fn password(&self) -> Option<String> {
        self.password.clone()
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(dbname) = &self.dbname {
            write!(f, "dbname={} ", dbname)?;
        }

        if let Some(host) = &self.host {
            write!(f, "host={} ", host)?;
        }

        if let Some(password) = &self.password {
            write!(f, "password={} ", password)?;
        }

        if let Some(port) = &self.port {
            write!(f, "port={} ", port)?;
        }

        if let Some(user) = &self.user {
            write!(f, "user={} ", user)?;
        }

        Ok(())
    }
}
