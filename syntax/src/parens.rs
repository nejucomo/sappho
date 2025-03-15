use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_parsable::primitive::bracketed;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::parseparams::ParseParams;
use crate::restrict::RestrictInto;
use crate::{BoxWise, ParensExpr};

impl<FX> ParsableWith<ParseParams<'_>> for ParensExpr<FX>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
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
