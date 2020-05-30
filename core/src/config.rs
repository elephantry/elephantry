#[derive(Clone, Debug)]
pub struct Config {
    pub host: Option<String>,
    pub user: Option<String>,
    pub dbname: Option<String>,
    pub port: Option<String>,
    pub password: Option<String>,
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
        Config {
            host: None,
            dbname: None,
            user: None,
            port: None,
            password: None,
        }
    }

    pub fn user(&self) -> String {
        get!(self.user, "PGUSER", std::env::var("USER").unwrap())
    }

    pub fn host(&self) -> String {
        get!(self.host, "PGHOST", "/run/postgresql".to_string())
    }

    pub fn dbname(&self) -> String {
        get!(self.dbname, "PGDATABASE", self.user())
    }

    pub fn port(&self) -> String {
        get!(self.port, "PGPORT", "5432".to_string())
    }

    pub fn password(&self) -> Option<String> {
        self.password
            .clone()
            .or_else(|| std::env::var("PGPASSWORD").ok())
            .or_else(|| {
                let pgpass = crate::PgPass::from_file();
                pgpass.find(
                    &self.host(),
                    &self.port(),
                    &self.dbname(),
                    &self.user(),
                )
            })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(password) = self.password() {
            write!(f, "password={} ", password)?;
        }

        write!(f, "host={} ", self.host())?;
        write!(f, "user={} ", self.user())?;
        write!(f, "dbname={} ", self.dbname())?;
        write!(f, "port={} ", self.port())
    }
}
