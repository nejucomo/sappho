use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::{ProcEffects, PureEffects, QueryEffects};
use sappho_rand_dcomp::{DistributionExt, WeightedCase};

use crate::AstFuzz;

impl Distribution<ProcEffects> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ProcEffects {
        self.map(|()| ProcEffects::Inquire)
            .weighted_case(3)
            .or(self.map(|()| ProcEffects::Invoke).weighted_case(1))
            .sample(rng)
    }
}

impl Distribution<QueryEffects> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> QueryEffects {
        QueryEffects::Inquire
    }
}

impl Distribution<PureEffects> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> PureEffects {
        unreachable!("PureEffects cannot be sampled")
    }
}
