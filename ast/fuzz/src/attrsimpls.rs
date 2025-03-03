use rand::distr::Distribution;
use rand::Rng;
use sappho_attrs::Attrs;
use sappho_identifier::RcId;

use crate::AstFuzz;

impl<T> Distribution<Attrs<T>> for AstFuzz
where
    AstFuzz: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Attrs<T> {
        let mut idmap = Attrs::default();
        while rng.random_ratio(2, 3) {
            let id = rng.sample::<RcId, _>(self);
            let value = rng.sample::<T, _>(self);
            // We ignore the result, because it's ok if we ignore the collision:
            let _ = idmap.define(id, value);
        }
        idmap
    }
}
