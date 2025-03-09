mod delimited;
mod error;
mod expr;
mod keyword;
mod listform;
mod restrict;
mod space;

use crate::error::Errors;
use sappho_source::LoadSource;

pub use self::error::LoadParseError;

pub fn parse<S, C>(loadsource: S) -> Result<sappho_ast::PureExpr, LoadParseError>
where
    S: LoadSource<C>,
    C: Clone + AsRef<str> + ToString,
{
    use chumsky::Parser;

    let scode = loadsource.load()?;

    self::expr::expression()
        .parse(scode.code().trim_end())
        .map_err(|bares| LoadParseError::Parse(Errors::attach_source(scode, bares)))
}

#[cfg(test)]
mod tests;
