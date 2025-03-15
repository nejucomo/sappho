use rand::distr::Distribution;
use sappho_ast_kernel::{BoxWise, Wise};
use sappho_ast_rich::{Ast, Expr};
use sappho_with_source::WithSource;

use crate::effectsimpls::FxFuzz;
use crate::AstFuzz;

impl<FX> Distribution<Wise<Ast, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Wise<Ast, FX> {
        Wise::from(rng.sample::<WithSource<Expr<FX>>, _>(self))
    }
}

impl<FX> Distribution<BoxWise<Ast, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> BoxWise<Ast, FX> {
        BoxWise::from(rng.sample::<Box<Wise<Ast, FX>>, _>(self))
    }
}
