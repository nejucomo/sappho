use rand::distr::Distribution;
use rand::Rng;
use sappho_listform::ListForm;

use crate::AstFuzz;

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
