use chumsky::Parser as _;
use sappho_syntax_parsable::{Parsable, Parser};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::Applications;

#[derive(Debug)]
pub struct Expr(Box<Applications>);

impl From<Applications> for Expr {
    fn from(apps: Applications) -> Self {
        Expr(Box::new(apps))
    }
}

impl Parsable for Expr {
    fn parser() -> impl Parser<Self> {
        Applications::parser().map(Expr::from)
    }
}

impl Unparse for Expr {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
