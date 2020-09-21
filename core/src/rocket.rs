use rocket_contrib::databases::{r2d2, DatabaseConfig, Poolable};

impl Poolable for crate::Connection {
    type Error = r2d2::Error;
    type Manager = crate::r2d2::ConnectionManager;

    fn pool(
        config: DatabaseConfig<'_>,
    ) -> Result<r2d2::Pool<Self::Manager>, Self::Error> {
        let manager = Self::Manager::new(config.url);

        r2d2::Pool::builder()
            .max_size(config.pool_size)
            .build(manager)
    }
}
