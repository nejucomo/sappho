pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Wrap(#[from] WrapError),
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
}

#[derive(Copy, Clone, Debug, thiserror::Error)]
pub enum WrapError {
    #[error("a line was too wide at {column} columns vs limit of {limit}")]
    TooWide { column: usize, limit: usize },
    #[error("a newline wrapped the unparse")]
    Newline,
}
