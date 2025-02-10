use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::{CoreExpr, Expr};
use sappho_rand_dcomp::{DistributionExt, WeightedCase};

use crate::AstFuzz;

impl<FX> Distribution<Expr<FX>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Expr<FX> {
        use Expr::*;

        // This is the single place where we reduce the recursion limit:
        let lower = self.next_lower_level();

        let rwf = lower.recursive_weight_factor();

        lower
            .map(Core)
            .weighted_case(1)
            .or(lower.map(Func).weighted_case(rwf))
            .or(lower.map(Query).weighted_case(rwf))
            .or(lower.map(Proc).weighted_case(rwf))
            .or(lower.map(List).weighted_case(rwf))
            .sample(rng)
    }
}

impl<FX> Distribution<CoreExpr<FX>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CoreExpr<FX> {
        use sappho_ast_core::CoreExpr::*;

        let rwf = self.recursive_weight_factor();

        self.map(Lit)
            .weighted_case(1)
            .or(self.map(Ref).weighted_case(1))
            .or(self.map(Object).weighted_case(rwf))
            .or(self.map(Let).weighted_case(rwf))
            .sample(rng)
    }
}
