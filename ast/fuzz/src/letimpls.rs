use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::{LetClause, LetExpr};

use crate::AstFuzz;

impl<FX> Distribution<LetExpr<FX>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LetExpr<FX> {
        LetExpr::new(rng.sample(self), rng.sample(self))
    }
}

impl<FX> Distribution<LetClause<FX>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LetClause<FX> {
        LetClause::new(rng.sample(self), rng.sample(self))
    }
}
