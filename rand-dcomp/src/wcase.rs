use rand::distr::Distribution;

use crate::{WeightedCaseBase, WeightedCaseOr};

pub trait WeightedCase<T>: Distribution<T> {
    fn or<'a, D>(&'a self, case: WeightedCaseBase<'a, D>) -> WeightedCaseOr<'a, D, Self> {
        WeightedCaseOr::new(case, self)
    }

    /// Returns the frequency ratio of this case
    fn weighted_ratio(&self) -> (u32, u32);
}
