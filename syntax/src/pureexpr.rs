use chumsky::Parser as _;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::proc::ProcExprParser;
use crate::restrict::RestrictInto;
use crate::{ProcExpr, PureExpr};

impl ParsableWith<ProcExprParser<'_>> for PureExpr {
    fn make_parser_with(pep: ProcExprParser<'_>) -> impl Parser<Self> {
        ProcExpr::parser_with(pep).try_map(|px, span| px.restrict(span))
    }
}

impl Unparse for PureExpr {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
