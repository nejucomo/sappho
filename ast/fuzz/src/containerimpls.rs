use rand::distr::Distribution;
use rand::Rng;

use crate::AstFuzz;

impl<T> Distribution<Box<T>> for AstFuzz
where
    AstFuzz: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Box<T> {
        Box::new(rng.sample(self))
    }
}

impl<T> Distribution<Option<T>> for AstFuzz
where
    AstFuzz: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<T> {
        if rng.random_ratio(1, 2) {
            Some(rng.sample(self))
        } else {
            None
        }
    }
}
