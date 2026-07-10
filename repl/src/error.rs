use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReplError {}

pub type ReplResult<T> = Result<T, ReplError>;
