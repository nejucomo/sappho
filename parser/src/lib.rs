mod comment;
mod delimited;
mod error;
mod expr;
mod keyword;
mod listform;
mod restrict;
mod space;

use chumsky::prelude::Recursive;
use sappho_ast::Expr;
use sappho_ast_effect::ProcEffect;
use sappho_source::LoadSource;

use crate::error::{BareError, Errors};

pub(crate) type RecExpr<'a> = Recursive<'a, char, Expr<ProcEffect>, BareError>;

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
