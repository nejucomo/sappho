use chumsky::Parser as _;
use derive_more::{From, TryInto};
use sappho_attrs::Attrs;
use sappho_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_listform::ListForm;
use sappho_parsable::{ParsableWith, Parser};
use sappho_primval::Num;
use sappho_unparse::{Stream, Unparse};

use crate::parseparams::ParseParams;
use crate::{
    Applications, BoxWise, FuncDef, Interactions, Let, Lookups, Match, ObjectDef, ProcDef,
    QueryDef, Wise,
};

/// The bare top-level expression without source annotation
#[derive(Debug, PartialEq, From, TryInto)]
pub enum Expr<FX>
where
    FX: Effect,
{
    #[from]
    Func(FuncDef),
    #[from]
    Query(QueryDef),
    #[from]
    Proc(ProcDef),
    #[from]
    Let(Let<FX>),
    #[from]
    Match(Match<FX>),
    #[from(
        Applications<FX>,
        Lookups<FX>,
        Interactions<FX>,
        &'static str,
        Num,
        ObjectDef<FX>,
        Attrs<Wise<FX>>,
        ListForm<Wise<FX>, BoxWise<FX>>,
    )]
    Applications(Applications<FX>),
}

impl<FX> ParsableWith<ParseParams<'_>> for Expr<FX>
where
    FX: Effect + RestrictFrom<ProcEffect>,
{
    fn make_parser_with(pep: ParseParams<'_>) -> impl Parser<Self> {
        use Expr::*;

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
        use Expr::*;

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
