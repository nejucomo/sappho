//! The _Rec_ursive _Core_ subset
use rand::distr::Distribution;
use rand::Rng;
use sappho_ast_core::{
    ApplicationExpr, EffectExpr, LetClause, LetExpr, LookupExpr, MatchClause, MatchExpr,
};
use sappho_ast_rich::{AstRich, Expr, Pattern};
use sappho_syntax_idstore::ArcId;

use crate::effectsimpls::FxFuzz;
use crate::AstFuzz;

impl<FX> Distribution<LetExpr<AstRich, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LetExpr<AstRich, FX> {
        LetExpr::new(
            rng.sample::<Vec<LetClause<AstRich, FX>>, _>(self),
            rng.sample::<Box<Expr<FX>>, _>(self),
        )
    }
}

impl<FX> Distribution<LetClause<AstRich, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LetClause<AstRich, FX> {
        LetClause::new(
            rng.sample::<Pattern, _>(self),
            rng.sample::<Box<Expr<FX>>, _>(self),
        )
    }
}

impl<FX> Distribution<MatchExpr<AstRich, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MatchExpr<AstRich, FX> {
        MatchExpr::new(
            rng.sample::<Box<Expr<FX>>, _>(self),
            rng.sample::<Vec<MatchClause<AstRich, FX>>, _>(self),
        )
    }
}

impl<FX> Distribution<MatchClause<AstRich, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MatchClause<AstRich, FX> {
        MatchClause::new(
            rng.sample::<Pattern, _>(self),
            rng.sample::<Box<Expr<FX>>, _>(self),
        )
    }
}

impl<FX> Distribution<ApplicationExpr<AstRich, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ApplicationExpr<AstRich, FX> {
        ApplicationExpr::new(
            rng.sample::<Box<Expr<FX>>, _>(self),
            rng.sample::<Box<Expr<FX>>, _>(self),
        )
    }
}

impl<FX> Distribution<LookupExpr<AstRich, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LookupExpr<AstRich, FX> {
        LookupExpr::new(
            rng.sample::<Box<Expr<FX>>, _>(self),
            rng.sample::<ArcId, _>(self),
        )
    }
}

impl<FX> Distribution<EffectExpr<AstRich, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EffectExpr<AstRich, FX> {
        EffectExpr::new(
            rng.sample::<FX, _>(self),
            rng.sample::<Box<Expr<FX>>, _>(self),
        )
    }
}
