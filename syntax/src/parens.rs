use chumsky::Parser as _;
use derive_more::From;
use sappho_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_kast::ProcWiseParser;
use sappho_parsable::primitive::bracketed;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};
use sappho_with_source::WithSource;

use crate::{BoxWise, Expr, SyntaxProvider, Wise};

/// # TODO
///
/// Rename to `Parens` for consistency.
#[derive(Clone, Debug, PartialEq, From)]
#[from(
    BoxWise<FX>,
    Wise<FX>,
    WithSource<Expr<FX>>,
)]
pub struct ParensExpr<FX>(BoxWise<FX>)
where
    FX: Effect;

impl<FX> ParensExpr<FX>
where
    FX: Effect,
{
    pub fn unwrap(self) -> BoxWise<FX> {
        self.0
    }
}

impl<FX> ParsableWith<ProcWiseParser<'_, SyntaxProvider>> for ParensExpr<FX>
where
    FX: Effect,
{
    fn make_parser_with(rec: ProcWiseParser<'_, SyntaxProvider>) -> impl Parser<Self> {
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

impl<FX> RestrictFrom<ParensExpr<ProcEffect>> for ParensExpr<FX>
where
    FX: Effect,
{
    fn restrict(src: ParensExpr<ProcEffect>) -> Result<ParensExpr<FX>, Restriction> {
        BoxWise::restrict(src.0).map(ParensExpr)
    }
}
