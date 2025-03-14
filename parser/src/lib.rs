mod delimited;
mod error;
mod expr;
mod listform;
mod restrict;
mod space;

use sappho_parsable::error::ParseError;
use sappho_source::LoadSource;

pub use self::error::LoadParseError;

pub fn parse<S>(loadsource: S) -> Result<sappho_ast::PureExpr, LoadParseError>
where
    S: LoadSource,
{
    use chumsky::Parser;

    let scode = loadsource.load()?;

    self::expr::expression()
        .parse(scode.code().trim_end())
        .map_err(|bares| ParseError::new(scode, bares).into())
}

#[cfg(test)]
mod tests;
