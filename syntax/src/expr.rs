use chumsky::Parser as _;
use sappho_ast_effect::Effect;
use sappho_parsable::{Parsable, ParsableWith, Parser, Recursive};
use sappho_unparse::{Stream, Unparse};

use crate::procexpr::ProcExprParser;
use crate::Expr::{self, *};
use crate::{FuncDef, ProcDef, ProcEffect, ProcExpr, QueryDef};

impl<FX> ParsableWith<ProcExprParser<'_>> for Expr<FX>
where
    FX: Effect,
{
    fn make_parser_with(expr: ProcExprParser<'_>) -> impl Parser<Self> {
        FuncDef::parser_with(expr.clone())
            .map(Func)
            .or(QueryDef::parser_with(expr.clone()).map(Query))
            .or(ProcDef::parser_with(expr.clone()).map(Proc))
            .or(crate::Let::parser_with(expr.clone()).map(Let))
            .or(crate::Match::parser_with(expr.clone()).map(Match))
            .or(crate::Applications::parser_with(expr).map(Applications))
            .try_map(|proc_expr, span| proc_expr.restrict(span))
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
