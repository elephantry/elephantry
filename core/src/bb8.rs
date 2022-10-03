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

#[async_trait::async_trait]
impl bb8::ManageConnection for ConnectionManager {
    type Connection = crate::Connection;
    type Error = crate::Error;

    async fn connect(&self) -> std::result::Result<Self::Connection, Self::Error> {
        crate::Connection::new(&self.dsn)
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> std::result::Result<(), Self::Error> {
        conn.r#async().execute("SELECT 1").await.map(|_| ())
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        conn.has_broken().unwrap_or(true)
    }
}

#[cfg(test)]
mod test {
    #[async_std::test]
    async fn bb8() {
        let manager = crate::bb8::ConnectionManager::new(&crate::test::dsn());
        let pool = bb8::Pool::builder().build(manager).await.unwrap();

        assert!(pool.get().await.is_ok());
    }
}
