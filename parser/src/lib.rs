mod error;
mod listform;
mod parser;
mod space;

pub use self::error::Error;

pub fn parse(src: &str) -> Result<saplang_ast::Expr, Vec<Error>> {
    use chumsky::Parser;

    self::parser::expression().parse(src)
}

#[cfg(test)]
mod tests;
