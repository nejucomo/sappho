use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::{Effect, Expr, ListExpr};
use sappho_listform::ListForm;

use crate::effectsimpls::FxFuzz;
use crate::AstFuzz;

impl<FX> Distribution<ListExpr<FX>> for AstFuzz
where
    FX: Effect + FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ListExpr<FX> {
        ListExpr::new(rng.sample::<ListForm<Expr<FX>, Box<Expr<FX>>>, _>(self))
    }
}

impl<X, T> Distribution<ListForm<X, T>> for AstFuzz
where
    AstFuzz: Distribution<X> + Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ListForm<X, T> {
        let body: Vec<X> = rng.sample(self);
        let optail = rng.sample(self);
        ListForm::new(body, optail)
    }
}
