mod delimited;
mod error;
mod expr;
mod listform;
mod restrict;

use chumsky::Parser as _;
use sappho_parsable::error::ParseError;
use sappho_source::LoadSource;

pub use self::error::LoadParseError;

pub fn parse<S, C>(loadsource: S) -> Result<sappho_ast::PureExpr, LoadParseError>
where
    S: LoadSource<C>,
    C: Clone + AsRef<str> + ToString,
{
    let scode = loadsource.load()?;

    self::expr::expression()
        .parse(scode.code().trim_end())
        .map_err(|bares| ParseError::new(scode.to_owned(), bares).into())
}

#[cfg(test)]
mod tests;
