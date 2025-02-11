mod baseimpls;
mod containerimpls;
mod effectsimpls;
mod exprimpls;
mod fqpimpls;
mod fuzz;
mod idmapimpls;
mod listimpls;
mod patternimpls;
mod recoreimpls;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use sappho_ast::{Expr, PureEffect};

/// Return `(seed, expr)` where `expr` is a randomly generated expression using `seed`
pub fn random_expr(max_depth: usize) -> (u64, Expr<PureEffect>) {
    let seed: u64 = rand::rng().random();
    let mut prng = StdRng::seed_from_u64(seed);
    let expr = prng.sample(AstFuzz::new(max_depth));
    (seed, expr)
}

pub use self::fuzz::AstFuzz;
