mod common;
mod pattern;
mod pure;
mod recursive;
mod universal;

use crate::Error;
use chumsky::Parser;
use saplang_ast::PureExpr;

pub(crate) fn expression() -> impl Parser<char, PureExpr, Error = Error> {
    self::pure::pure_expr().then_ignore(chumsky::primitive::end())
}
