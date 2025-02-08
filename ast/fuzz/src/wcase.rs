use std::marker::PhantomData;

use derive_new::new;
use rand::distr::Distribution;
use rand::Rng;

#[derive(Debug, new)]
pub struct WeightedCaseBase<D, F, S, T> {
    dist: D,
    #[allow(dead_code)]
    weight: u32,
    f: F,
    #[new(default)]
    ph: PhantomData<(S, T)>,
}

impl<D, F, S, T> Distribution<T> for WeightedCaseBase<D, F, S, T>
where
    D: Distribution<S>,
    F: Fn(S) -> T,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        (self.f)(self.dist.sample(rng))
    }
}
