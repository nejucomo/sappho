mod delimited;
mod error;
mod expr;
mod keyword;
mod listform;
mod restrict;
mod space;

use std::path::PathBuf;

pub use self::error::Errors;

pub fn parse(path: Option<PathBuf>, src: &str) -> Result<sappho_ast::PureExpr, Errors> {
    use chumsky::Parser;

    self::expr::expression()
        .parse(src.trim_end())
        .map_err(|bares| Errors::attach_source(path, src, bares))
}

#[cfg(test)]
mod tests;
