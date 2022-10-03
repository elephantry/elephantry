pub struct ConnectionManager {
    dsn: String,
}

impl ConnectionManager {
    pub fn new(dsn: &str) -> Self {
        Self {
            dsn: dsn.to_string(),
        }
    }
}

impl r2d2::ManageConnection for ConnectionManager {
    type Connection = crate::Connection;
    type Error = crate::Error;

    fn connect(&self) -> std::result::Result<Self::Connection, Self::Error> {
        crate::Connection::new(&self.dsn)
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> std::result::Result<(), Self::Error> {
        conn.execute("SELECT 1").map(|_| ())
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        conn.has_broken().unwrap_or(true)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn r2d2() {
        let manager = crate::r2d2::ConnectionManager::new(&crate::test::dsn());
        let pool = r2d2::Pool::builder().max_size(1).build(manager).unwrap();

        assert!(pool.get().is_ok())
    }
}
