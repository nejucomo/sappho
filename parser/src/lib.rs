mod delimited;
mod error;
mod expr;
mod listform;
mod restrict;
mod space;

use sappho_parsable::error::ParseError;
use sappho_source::LoadSource;

pub use self::error::LoadParseError;

pub fn parse<S, C>(loadsource: S) -> Result<sappho_ast::PureExpr, LoadParseError>
where
    S: LoadSource<C>,
    C: Clone + AsRef<str> + ToString,
{
    use chumsky::Parser;

    let (source, code) = loadsource.load()?.into();

    self::expr::expression()
        .parse(code.as_ref().trim_end())
        .map_err(|bares| ParseError::new(source, bares).into())
}

#[cfg(test)]
mod tests;
