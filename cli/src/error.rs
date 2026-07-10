use derive_more::From;
use thiserror::Error;

#[derive(Debug, Error, From)]
pub enum CliError<'e> {
    #[error(transparent)]
    Repl(sappho_repl::ReplError),
    #[error(transparent)]
    Eval(sappho_interpreter::Error<'e>),
    #[error("{0}")]
    LoadParse(sappho_parser::LoadParseError<'e>),
}

pub type CliResult<'e, T> = Result<T, CliError<'e>>;
