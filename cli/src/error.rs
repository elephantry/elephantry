#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Env(#[from] envir::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Sql(#[from] elephantry::Error),
}
