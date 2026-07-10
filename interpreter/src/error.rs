use derive_more::From;
use thiserror::Error;

#[derive(Debug, Error, From)]
pub enum Error<'a> {
    #[error("{0}")]
    LoadParse(sappho_parser::LoadParseError<'a>),
    #[error("eval error: {0}")]
    Eval(sappho_eval::Error),
}

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;
