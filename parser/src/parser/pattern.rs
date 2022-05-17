use crate::Error;
use chumsky::{text, Parser};
use saplang_ast::Pattern;

pub(crate) fn pattern() -> impl Parser<char, Pattern, Error = Error> {
    text::ident()
}
