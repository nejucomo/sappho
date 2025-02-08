use derive_new::new;
use rand::distr::Distribution;
use rand::Rng;

use crate::WeightedCase;

#[derive(Debug, new)]
pub struct WeightedCaseBase<'a, D>
where
    D: ?Sized,
{
    dist: &'a D,
    pub(crate) freq: u32,
}

impl<D, T> Distribution<T> for WeightedCaseBase<'_, D>
where
    D: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        self.dist.sample(rng)
    }
}

impl<D, T> WeightedCase<T> for WeightedCaseBase<'_, D>
where
    D: Distribution<T>,
{
    fn weighted_ratio(&self) -> (u32, u32) {
        (self.freq, self.freq)
    }
}
