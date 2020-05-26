#[derive(Clone, Debug)]
pub struct Config {
    pub host: Option<String>,
    pub user: Option<String>,
    pub dbname: Option<String>,
    pub port: Option<String>,
    pub password: Option<String>,
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
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let user = self.user
            .clone()
            .or(std::env::var("PGUSER").ok())
            .unwrap_or_else(|| std::env::var("USER").unwrap());

        let host = self.host
            .clone()
            .or(std::env::var("PGHOST").ok())
            .unwrap_or_else(|| "/run/postgresql".to_string());

        let dbname = self.dbname
            .clone()
            .or(std::env::var("PGDATABASE").ok())
            .unwrap_or_else(|| user.clone());

        let port = self.port
            .clone()
            .or(std::env::var("PGPORT").ok())
            .unwrap_or_else(|| "5432".to_string());

        let password = self.password
            .clone()
            .or(std::env::var("PGPASSWORD").ok())
            .or_else(|| {
                let pgpass = crate::PgPass::from_file();
                pgpass.find(&host, &port, &dbname, &user)
            });

        if let Some(password) = password {
            write!(f, "password={} ", password)?;
        }

        write!(f, "host={} ", host)?;
        write!(f, "user={} ", user)?;
        write!(f, "dbname={} ", dbname)?;
        write!(f, "port={} ", port)
    }
}
