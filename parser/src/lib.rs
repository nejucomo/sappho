mod delimited;
mod error;
mod expr;
mod keyword;
mod listform;
mod restrict;
mod space;

use sappho_code_origin::CodeOrigin;

pub use self::error::ParseErrors;

pub fn parse<'a>(co: CodeOrigin<'a>) -> Result<sappho_ast::PureExpr, ParseErrors<'a>> {
    use chumsky::Parser;

    self::expr::expression()
        .parse(co.code().trim_end())
        .map_err(|bares| ParseErrors::attach_source(co, bares))
}

#[cfg(test)]
mod tests;
