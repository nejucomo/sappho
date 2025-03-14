use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_parsable::primitive::bracketed;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::parseparams::ParseParams;
use crate::{BoxWise, ParensExpr};

impl ParsableWith<ParseParams<'_>> for ParensExpr<ProcEffect> {
    fn make_parser_with(rec: ParseParams<'_>) -> impl Parser<Self> {
        bracketed(['(', ')'], BoxWise::parser_with(rec)).map(ParensExpr)
    }
}

impl<FX> Unparse for ParensExpr<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
