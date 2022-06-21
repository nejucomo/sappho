use crate::error::BareError;
use crate::expr::universal::literal;
use chumsky::{text, Parser};
use sappho_ast::Pattern;

pub(crate) fn pattern() -> impl Parser<char, Pattern, Error = BareError> {
    use Pattern::*;

    text::ident().map(Bind).or(literal().map(LitEq))
}
