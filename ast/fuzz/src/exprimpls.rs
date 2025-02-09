use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::Expr;

use crate::AstFuzz;

impl<FX> Distribution<Expr<FX>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Expr<FX> {
        self.map(Expr::Proc).sample(rng)
    }
}
