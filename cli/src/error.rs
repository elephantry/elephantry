#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Sql(#[from] elephantry::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
}
