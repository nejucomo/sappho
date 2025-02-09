use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::Expr;
use sappho_rand_dcomp::{DistributionExt, WeightedCase};

use crate::AstFuzz;

impl<FX> Distribution<Expr<FX>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Expr<FX> {
        self.map(Expr::Func)
            .weighted_case(1)
            .or(self.map(Expr::Query).weighted_case(1))
            .or(self.map(Expr::Proc).weighted_case(1))
            .or(self.map(Expr::List).weighted_case(1))
            .sample(rng)
    }
}
