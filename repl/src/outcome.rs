use derive_more::From;
use sappho_eval::ValRef;
use sappho_parser::ParseErrors;
use thiserror::Error;

pub(crate) type ExecOutcome = Result<ValRef, ExecError>;

#[derive(Debug, Error, From)]
pub(crate) enum ExecError {
    #[error("{0}")]
    Parse(String),
    #[error("{0}")]
    Eval(sappho_eval::Error),
}

impl From<ParseErrors<'_>> for ExecError {
    fn from(e: ParseErrors<'_>) -> Self {
        ExecError::Parse(e.to_string())
    }
}
