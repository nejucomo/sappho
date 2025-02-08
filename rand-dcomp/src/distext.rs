use rand::distr::Distribution;

use crate::WeightedCaseBase;

pub trait DistributionExt<T>: Sized + Distribution<T> {
    /// Begin constructing a set of weighted cases, giving `self` a frequency of `freq`
    ///
    /// # Example
    ///
    /// ```
    /// use rand::distr::Distribution;
    /// use sappho_rand_dcomp::{DistributionExt, WeightedCase};
    ///
    /// #[derive(Debug)]
    /// enum Fruit {
    ///     Banana,
    ///     Grape(GrapeColor),
    /// }
    /// use Fruit::*;
    ///
    /// #[derive(Debug)]
    /// enum GrapeColor {
    ///     Green,
    ///     Red,
    /// }
    /// use GrapeColor::*;
    ///
    /// struct FruitSampler;
    ///
    /// impl Distribution<GrapeColor> for FruitSampler {
    ///     fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> GrapeColor {
    ///         // Red grapes are twice as likely as green grapes:
    ///         self.map(|()| Green).weighted_case(1)
    ///             .or(self.map(|()| Red).weighted_case(2))
    ///             .sample(rng)
    ///     }
    /// }
    ///
    /// impl Distribution<Fruit> for FruitSampler {
    ///     fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Fruit {
    ///         // On average, for every 3 bananas there are 2 grapes:
    ///         self.map(|()| Banana).weighted_case(3)
    ///             // Notice we pass `Grape` directly rather than a closure:
    ///             .or(self.map(Grape).weighted_case(2))
    ///             .sample(rng)
    ///     }
    /// }
    ///
    /// impl Distribution<()> for FruitSampler {
    ///     fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> () {
    ///         ()
    ///     }
    /// }
    ///
    ///
    /// use rand::SeedableRng;
    ///
    /// // Notice: we seed with a constant for reproducible results. However,
    /// // different versions of `StdRng` may produce different results for the
    /// // same seed.
    /// let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    ///
    /// assert!(matches!(FruitSampler.sample(&mut rng), Banana));
    /// assert!(matches!(FruitSampler.sample(&mut rng), Banana));
    /// assert!(matches!(FruitSampler.sample(&mut rng), Grape(Red)));
    /// assert!(matches!(FruitSampler.sample(&mut rng), Grape(Green)));
    /// assert!(matches!(FruitSampler.sample(&mut rng), Grape(Green)));
    /// assert!(matches!(FruitSampler.sample(&mut rng), Banana));
    /// ```
    fn weighted_case(&self, freq: u32) -> WeightedCaseBase<Self> {
        WeightedCaseBase::new(self, freq)
    }
}

impl<D, T> DistributionExt<T> for D where D: Distribution<T> {}
