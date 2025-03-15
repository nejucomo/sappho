//! The _Rec_ursive _Core_ subset
use rand::distr::Distribution;
use rand::Rng;
use sappho_ast_kernel::{
    ApplicationExpr, BoxWise, Interaction, LetClause, LetExpr, LookupExpr, MatchClause, MatchExpr,
};
use sappho_ast_rich::{Ast, Pattern};
use sappho_identifier::RcId;

use crate::effectsimpls::FxFuzz;
use crate::AstFuzz;

impl<FX> Distribution<LetExpr<Ast, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LetExpr<Ast, FX> {
        LetExpr::new(
            rng.sample::<Vec<LetClause<Ast, FX>>, _>(self),
            rng.sample::<BoxWise<Ast, FX>, _>(self),
        )
    }
}

impl<FX> Distribution<LetClause<Ast, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LetClause<Ast, FX> {
        LetClause::new(
            rng.sample::<Pattern, _>(self),
            rng.sample::<BoxWise<Ast, FX>, _>(self),
        )
    }
}

impl<FX> Distribution<MatchExpr<Ast, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MatchExpr<Ast, FX> {
        MatchExpr::new(
            rng.sample::<BoxWise<Ast, FX>, _>(self),
            rng.sample::<Vec<MatchClause<Ast, FX>>, _>(self),
        )
    }
}

impl<FX> Distribution<MatchClause<Ast, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MatchClause<Ast, FX> {
        MatchClause::new(
            rng.sample::<Pattern, _>(self),
            rng.sample::<BoxWise<Ast, FX>, _>(self),
        )
    }
}

impl<FX> Distribution<ApplicationExpr<Ast, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ApplicationExpr<Ast, FX> {
        ApplicationExpr::new(
            rng.sample::<BoxWise<Ast, FX>, _>(self),
            rng.sample::<BoxWise<Ast, FX>, _>(self),
        )
    }
}

impl<FX> Distribution<LookupExpr<Ast, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LookupExpr<Ast, FX> {
        LookupExpr::new(
            rng.sample::<BoxWise<Ast, FX>, _>(self),
            rng.sample::<RcId, _>(self),
        )
    }
}

impl<FX> Distribution<Interaction<Ast, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Interaction<Ast, FX> {
        Interaction::new(
            rng.sample::<FX, _>(self),
            rng.sample::<BoxWise<Ast, FX>, _>(self),
        )
    }
}
