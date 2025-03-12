use chumsky::Parser as _;
use sappho_ast_effect::ProcEffect;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::proc::ProcExprParser;
use crate::restrict::RestrictInto;
use crate::spanned::Spanned;
use crate::{Expr, PureExpr};

impl ParsableWith<ProcExprParser<'_>> for PureExpr {
    fn make_parser_with(pep: ProcExprParser<'_>) -> impl Parser<Self> {
        Spanned::parser_with(pep)
            .try_map(|expr: Spanned<Expr<ProcEffect>>, span| expr.restrict(span))
            .map(Self)
    }
}

impl Unparse for PureExpr {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
