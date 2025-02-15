use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::{
    ApplicationExpr, Effect, EffectExpr, Expr, FuncDef, Identifier, LetExpr, ListExpr, Literal,
    LookupExpr, MatchExpr, ObjectDef, ProcDef, QueryDef,
};
use sappho_rand_dcomp::{DistributionExt, WeightedCase};

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

        <Self as Distribution<Literal>>::map(*self, Lit)
            .weighted_case(1)
            .or(<Self as Distribution<Identifier>>::map(*self, Ref).weighted_case(3))
            .or(<Self as Distribution<ObjectDef<FX>>>::map(*self, Object).weighted_case(rwf))
            .or(<Self as Distribution<FuncDef>>::map(lower, Func).weighted_case(rwf))
            .or(<Self as Distribution<QueryDef>>::map(lower, Query).weighted_case(rwf))
            .or(<Self as Distribution<ProcDef>>::map(lower, Proc).weighted_case(rwf))
            .or(<Self as Distribution<ListExpr<FX>>>::map(lower, List).weighted_case(rwf))
            .or(<Self as Distribution<LetExpr<FX>>>::map(*self, Let).weighted_case(rwf))
            .or(<Self as Distribution<MatchExpr<FX>>>::map(*self, Match).weighted_case(rwf))
            .or(
                <Self as Distribution<ApplicationExpr<FX>>>::map(*self, Application)
                    .weighted_case(rwf),
            )
            .or(<Self as Distribution<LookupExpr<FX>>>::map(*self, Lookup).weighted_case(rwf))
            .or(<Self as Distribution<EffectExpr<FX>>>::map(*self, Effect)
                .weighted_case(rwf * FX::fuzz_weight_factor()))
            .sample(rng)
    }
}
