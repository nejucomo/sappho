use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::parseparams::ParseParams;
use crate::Expr::{self, *};
use crate::{FuncDef, ProcDef, QueryDef};

impl<FX> ParsableWith<ParseParams<'_>> for Expr<FX>
where
    FX: Effect + RestrictFrom<ProcEffect>,
{
    fn make_parser_with(pep: ParseParams<'_>) -> impl Parser<Self> {
        FuncDef::parser_with(pep.clone())
            .map(Func)
            .or(QueryDef::parser_with(pep.clone()).map(Query))
            .or(ProcDef::parser_with(pep.clone()).map(Proc))
            .or(crate::Let::parser_with(pep.clone()).map(Let))
            .or(crate::Match::parser_with(pep.clone()).map(Match))
            .or(crate::Applications::parser_with(pep).map(Applications))
            // The primary enabler of trailing whitespace
            .then_opt_space()
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

impl<FX> RestrictFrom<Expr<ProcEffect>> for Expr<FX>
where
    FX: Effect,
{
    fn restrict(src: Expr<ProcEffect>) -> Result<Expr<FX>, Restriction> {
        use Expr::*;

        match src {
            Func(x) => Ok(Func(x)),
            Query(x) => Ok(Query(x)),
            Proc(x) => Ok(Proc(x)),
            Let(x) => crate::Let::restrict(x).map(Let),
            Match(x) => crate::Match::restrict(x).map(Match),
            Applications(x) => crate::Applications::restrict(x).map(Applications),
        }
    }
}
