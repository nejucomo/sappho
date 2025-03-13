use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::Expr::{self, *};
use crate::{FuncDef, ProcDef, QueryDef, SEParser};

impl ParsableWith<SEParser<'_>> for Expr<ProcEffect> {
    fn make_parser_with(pep: SEParser<'_>) -> impl Parser<Self> {
        FuncDef::parser_with(pep.clone())
            .map(Func)
            .or(QueryDef::parser_with(pep.clone()).map(Query))
            .or(ProcDef::parser_with(pep.clone()).map(Proc))
            .or(crate::Let::parser_with(pep.clone()).map(Let))
            .or(crate::Match::parser_with(pep.clone()).map(Match))
            .or(crate::Applications::parser_with(pep).map(Applications))
    }
}

impl<FX> Unparse for Expr<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        match self {
            Func(x) => x.unparse_into(s),
            Query(x) => x.unparse_into(s),
            Proc(x) => x.unparse_into(s),
            Let(x) => x.unparse_into(s),
            Match(x) => x.unparse_into(s),
            Applications(x) => x.unparse_into(s),
        }
    }
}
