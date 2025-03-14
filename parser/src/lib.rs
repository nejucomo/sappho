mod delimited;
mod error;
mod expr;
mod listform;
mod restrict;
mod space;

use sappho_parsable::error::ParseError;
use sappho_source::Source;

pub use self::error::LoadParseError;

pub fn parse<S>(source: S) -> Result<sappho_ast_rich::PureExpr, LoadParseError>
where
    Source: From<S>,
{
    use chumsky::Parser;

    let scode = Source::from(source).load()?;

    self::expr::expression()
        .parse(scode.code().trim_end())
        .map_err(|bares| ParseError::new(scode, bares).into())
}

#[cfg(test)]
mod tests;
