use derive_more::From;
use sappho_parser::ParseErrors;
use thiserror::Error;

#[derive(Debug, Error, From)]
pub enum Error<'a> {
    #[error("{0}")]
    Parse(ParseErrors<'a>),
    #[error("eval error: {0}")]
    Eval(sappho_eval::Error),
}

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;
