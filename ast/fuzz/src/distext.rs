use rand::distr::Distribution;

use crate::WeightedCaseBase;

pub trait DistributionExt<T>: Sized + Distribution<T> {
    fn weighted_case<F, S>(self, weight: u32, f: F) -> WeightedCaseBase<Self, F, S, T> {
        WeightedCaseBase::new(self, weight, f)
    }
}

impl<D, T> DistributionExt<T> for D where D: Distribution<T> {}
