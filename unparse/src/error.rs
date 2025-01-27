pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unparsed output wrapped")]
    Wrapped,
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
}
