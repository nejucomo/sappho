use derive_more::From;
use sappho_eval::ValRef;
use thiserror::Error;

pub(crate) type ExecOutcome = Result<ValRef, ExecError>;

#[derive(Debug, Error, From)]
pub(crate) enum ExecError {
    #[error("{0}")]
    Eval(sappho_eval::Error),
}
