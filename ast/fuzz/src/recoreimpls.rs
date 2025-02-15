//! The _Rec_ursive _Core_ subset
use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::{
    ApplicationExpr, EffectExpr, Expr, Identifier, LetClause, LetExpr, LookupExpr, MatchClause,
    MatchExpr, Pattern,
};

use crate::effectsimpls::FxFuzz;
use crate::AstFuzz;

impl<FX> Distribution<LetExpr<FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LetExpr<FX> {
        LetExpr::new(
            rng.sample::<Vec<LetClause<FX>>, _>(self),
            rng.sample::<Box<Expr<FX>>, _>(self),
        )
    }
}

impl<FX> Distribution<LetClause<FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LetClause<FX> {
        LetClause::new(
            rng.sample::<Pattern, _>(self),
            rng.sample::<Box<Expr<FX>>, _>(self),
        )
    }
}

impl<FX> Distribution<MatchExpr<FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MatchExpr<FX> {
        MatchExpr::new(
            rng.sample::<Box<Expr<FX>>, _>(self),
            rng.sample::<Vec<MatchClause<FX>>, _>(self),
        )
    }
}

impl<FX> Distribution<MatchClause<FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MatchClause<FX> {
        MatchClause::new(
            rng.sample::<Pattern, _>(self),
            rng.sample::<Box<Expr<FX>>, _>(self),
        )
    }
}

impl<FX> Distribution<ApplicationExpr<FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ApplicationExpr<FX> {
        ApplicationExpr::new(
            rng.sample::<Box<Expr<FX>>, _>(self),
            rng.sample::<Box<Expr<FX>>, _>(self),
        )
    }
}

impl<FX> Distribution<LookupExpr<FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LookupExpr<FX> {
        LookupExpr::new(
            rng.sample::<Box<Expr<FX>>, _>(self),
            rng.sample::<Identifier, _>(self),
        )
    }
}

impl<FX> Distribution<EffectExpr<FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EffectExpr<FX> {
        EffectExpr::new(
            rng.sample::<FX, _>(self),
            rng.sample::<Box<Expr<FX>>, _>(self),
        )
    }
}
