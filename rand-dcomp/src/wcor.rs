use derive_new::new;
use rand::distr::Distribution;

use crate::{WeightedCase, WeightedCaseBase};

#[derive(Debug, new)]
pub struct WeightedCaseOr<'a, D, P>
where
    D: ?Sized,
    P: ?Sized,
{
    case: WeightedCaseBase<'a, D>,
    parent: &'a P,
}

impl<D, P, T> Distribution<T> for WeightedCaseOr<'_, D, P>
where
    D: Distribution<T>,
    P: WeightedCase<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> T {
        let (num, denom) = self.weighted_ratio();
        if rng.random_ratio(num, denom) {
            self.case.sample(rng)
        } else {
            self.parent.sample(rng)
        }
    }
}

impl<D, P, T> WeightedCase<T> for WeightedCaseOr<'_, D, P>
where
    D: Distribution<T>,
    P: WeightedCase<T>,
{
    fn weighted_ratio(&self) -> (u32, u32) {
        let f = self.case.freq;
        let (_, predenom) = self.parent.weighted_ratio();
        (f, f + predenom)
    }
}
