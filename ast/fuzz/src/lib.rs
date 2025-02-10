mod baseimpls;
mod containerimpls;
mod effectsimpls;
mod exprimpls;
mod fqpimpls;
mod fuzz;
mod idmapimpls;
mod letimpls;
mod listimpls;
mod patternimpls;

use rand::Rng;
use sappho_ast::PureExpr;

pub fn random_expr(max_depth: usize) -> PureExpr {
    rand::rng().sample(AstFuzz::new(max_depth))
}

pub use self::fuzz::AstFuzz;
