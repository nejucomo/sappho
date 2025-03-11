use chumsky::Parser as _;
use sappho_parsable::{Parsable, ParsableWith, Parser, Recursive};
use sappho_unparse::{Stream, Unparse};

use crate::Expr::{self, *};
use crate::{FuncDef, ProcDef, ProcEffect, ProcExpr, QueryDef};

impl<FX> Parsable for Expr<FX>
where
    FX: Parsable,
{
    fn parser() -> impl Parser<Self> {
        chumsky::recursive::recursive(ProcExpr::with_parser)
            .try_map(|px, _span| Self::restrict_from(px))
    }
}

impl ParsableWith<Recursive<'_, ProcExpr>> for ProcExpr {
    fn make_parser_with(expr: Recursive<'_, ProcExpr>) -> impl Parser<Self> {
        FuncDef::parser()
            .map(Func)
            .or(QueryDef::parser().map(Query))
            .or(ProcDef::parser().map(Proc))
            .or(crate::Let::parser().map(Let))
            .or(crate::Match::parser().map(Match))
            .or(crate::Applications::parser().map(Applications))
    }
}

impl<FX> Unparse for Expr<FX>
where
    FX: Unparse,
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
