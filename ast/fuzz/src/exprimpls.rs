use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::{CoreExpr, Expr};
use sappho_rand_dcomp::{DistributionExt, WeightedCase};

use crate::AstFuzz;

impl<FX> Distribution<Expr<FX>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Expr<FX> {
        use Expr::*;

        self.map(Core)
            .weighted_case(1)
            .or(self.map(Func).weighted_case(1))
            .or(self.map(Query).weighted_case(1))
            .or(self.map(Proc).weighted_case(1))
            .or(self.map(List).weighted_case(1))
            .sample(rng)
    }
}

impl<FX> Distribution<CoreExpr<FX>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CoreExpr<FX> {
        use sappho_ast_core::CoreExpr::*;

        self.map(Lit)
            .weighted_case(1)
            .or(self.map(Ref).weighted_case(1))
            .sample(rng)
    }
}
