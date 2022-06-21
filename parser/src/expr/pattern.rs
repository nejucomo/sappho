use crate::error::BareError;
use chumsky::{text, Parser};
use sappho_ast::Pattern;

pub(crate) fn pattern() -> impl Parser<char, Pattern, Error = BareError> {
    text::ident().map(Pattern::Bind)
}
