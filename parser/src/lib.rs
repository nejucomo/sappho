mod delimited;
mod error;
mod keyword;
mod listform;
mod parser;
mod restrict;
mod space;

pub use self::error::Errors;

pub fn parse(src: &str) -> Result<saplang_ast::PureExpr, Errors> {
    use chumsky::Parser;

    self::parser::expression()
        .parse(src)
        .map_err(|bares| Errors::attach_source(src, bares))
}

#[cfg(test)]
mod tests;
