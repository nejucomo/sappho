use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReplError {
    #[error(transparent)]
    Stdio(#[from] std::io::Error),
}

pub type ReplResult<T> = Result<T, ReplError>;
