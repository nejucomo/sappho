use chumsky::prelude::just;
use chumsky::Parser as _;
use sappho_keyword::Keyword::Fn;
use sappho_parsable::primitive::space;
use sappho_parsable::{Parsable, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::{Expr, FuncDef, Pattern};

impl Parsable for FuncDef {
    fn parser() -> impl Parser<Self> {
        Fn.parse()
            .then_space()
            .ignore_then(Pattern::parser())
            .then_ignore(just("->").delimited_by(space(), space()))
            .then(Expr::parser())
            .map(FuncDef::new)
            .labelled("fn definition")
    }
}

impl Unparse for FuncDef {
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&Fn);
        s.write(" ");
        s.write(&self.argpat);
        s.write(" -> ");
        s.write(&self.body);
    }
}
