use rand::distr::Distribution;
use rand::Rng;
use sappho_ast_core::{
    ApplicationExpr, CoreExpr, EffectExpr, FuncDef, LetExpr, Literal, LookupExpr, MatchExpr,
    ObjectDef, ProcDef, QueryDef,
};
use sappho_ast_effect::Effect;
use sappho_ast_rich::{AstRich, Expr, ListExpr};
use sappho_rand_dcomp::{DistributionExt, WeightedCase};
use sappho_syntax_idstore::ArcId;

use crate::effectsimpls::FxFuzz;
use crate::AstFuzz;

impl<FX> Distribution<Expr<FX>> for AstFuzz
where
    FX: Effect + FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Expr<FX> {
        use Expr::*;

        // This is the single place where we reduce the recursion limit:
        let lower = self.next_lower_level();
        let rwf = lower.recursive_weight_factor();

        <Self as Distribution<CoreExpr<AstRich, FX>>>::map(lower, Core)
            .weighted_case(1)
            .or(<Self as Distribution<FuncDef<AstRich>>>::map(lower, Func).weighted_case(rwf))
            .or(<Self as Distribution<QueryDef<AstRich>>>::map(lower, Query).weighted_case(rwf))
            .or(<Self as Distribution<ProcDef<AstRich>>>::map(lower, Proc).weighted_case(rwf))
            .or(<Self as Distribution<ListExpr<FX>>>::map(lower, List).weighted_case(rwf))
            .sample(rng)
    }
}

impl<FX> Distribution<CoreExpr<AstRich, FX>> for AstFuzz
where
    FX: Effect + FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CoreExpr<AstRich, FX> {
        use sappho_ast_core::CoreExpr::*;

        let rwf = self.recursive_weight_factor();
        let fxwf = FX::fuzz_weight_factor();

        <Self as Distribution<Literal>>::map(*self, Lit)
            .weighted_case(1)
            .or(<Self as Distribution<ArcId>>::map(*self, Ref).weighted_case(3))
            .or(
                <Self as Distribution<ObjectDef<AstRich, FX>>>::map(*self, Object)
                    .weighted_case(rwf),
            )
            .or(<Self as Distribution<LetExpr<AstRich, FX>>>::map(*self, Let).weighted_case(rwf))
            .or(
                <Self as Distribution<MatchExpr<AstRich, FX>>>::map(*self, Match)
                    .weighted_case(rwf),
            )
            .or(
                <Self as Distribution<ApplicationExpr<AstRich, FX>>>::map(*self, Application)
                    .weighted_case(rwf),
            )
            .or(
                <Self as Distribution<LookupExpr<AstRich, FX>>>::map(*self, Lookup)
                    .weighted_case(rwf),
            )
            .or(
                <Self as Distribution<EffectExpr<AstRich, FX>>>::map(*self, Effect)
                    .weighted_case(rwf * fxwf),
            )
            .sample(rng)
    }
}
