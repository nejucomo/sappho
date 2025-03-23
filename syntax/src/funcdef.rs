use chumsky::prelude::just;
use chumsky::Parser as _;
use derive_new::new;
use sappho_keyword::Keyword::Fn;
use sappho_parsable::primitive::space;
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::parseparams::ParseParams;
use crate::{Pattern, PureExpr};

#[derive(Debug, PartialEq, new)]
pub struct FuncDef {
    #[new(into)]
    argpat: Pattern,
    #[new(into)]
    body: PureExpr,
}

impl ParsableWith<ParseParams<'_>> for FuncDef {
    fn make_parser_with(pep: ParseParams<'_>) -> impl Parser<Self> {
        Fn.parse()
            .then_space()
            .ignore_then(Pattern::parser())
            .then_ignore(just("->").delimited_by(space(), space()))
            .then(PureExpr::parser_with(pep))
            .map(|(argpat, body)| FuncDef::new(argpat, body))
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
