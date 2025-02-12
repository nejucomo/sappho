mod delimited;
mod error;
mod expr;
mod keyword;
mod listform;
mod restrict;
mod space;

use sappho_source::LoadSource;

use crate::error::Errors;

pub use self::error::LoadParseError;

pub fn parse<'a, S>(sourceloader: S) -> Result<sappho_ast::PureExpr, LoadParseError<'a>>
where
    S: LoadSource<'a>,
{
    use chumsky::Parser;

    let source = sourceloader.load()?;

    self::expr::expression()
        .parse(source.text().trim_end())
        .map_err(|bares| LoadParseError::Parse(Errors::attach_source(source, bares)))
}

#[cfg(test)]
mod tests;
