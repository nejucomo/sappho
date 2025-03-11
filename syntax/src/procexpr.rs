use chumsky::Parser as _;
use sappho_parsable::{Parsable, ParsableWith, Parser, Recursive};
use sappho_unparse::{Stream, Unparse};

use crate::spanned::Spanned;
use crate::ProcExpr;

impl Parsable for ProcExpr {
    fn parser() -> impl Parser<Self> {
        chumsky::recursive::recursive(ProcExpr::parser_with)
    }
}

pub(crate) type ProcExprParser<'a> = Recursive<'a, ProcExpr>;

impl ParsableWith<ProcExprParser<'_>> for ProcExpr {
    fn make_parser_with(param: Recursive<'_, ProcExpr>) -> impl Parser<Self> {
        Spanned::parser_with(param).map(ProcExpr)
    }
}

impl Unparse for ProcExpr {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s);
    }
}
