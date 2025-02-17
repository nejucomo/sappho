use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::Identifier;
use sappho_attrs::Attrs;

use crate::AstFuzz;

impl<T> Distribution<Attrs<T>> for AstFuzz
where
    AstFuzz: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Attrs<T> {
        let mut idmap = Attrs::default();
        while rng.random_ratio(2, 3) {
            let id = rng.sample::<Identifier, _>(self);
            let value = rng.sample::<T, _>(self);
            // We ignore the result, because it's ok if we ignore the collision:
            let _ = idmap.define(id, value);
        }
        idmap
    }
}
