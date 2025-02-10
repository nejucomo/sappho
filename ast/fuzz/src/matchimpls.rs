use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::{MatchClause, MatchExpr};

use crate::AstFuzz;

impl<FX> Distribution<MatchExpr<FX>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MatchExpr<FX> {
        MatchExpr::new(rng.sample(self), rng.sample(self))
    }
}

impl<FX> Distribution<MatchClause<FX>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MatchClause<FX> {
        MatchClause::new(rng.sample(self), rng.sample(self))
    }
}
