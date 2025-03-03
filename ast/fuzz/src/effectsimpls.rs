use rand::distr::Distribution;
use rand::Rng;
use sappho_ast_effect::{Effect, ProcEffect, PureEffect, QueryEffect};
use sappho_rand_dcomp::{DistributionExt, WeightedCase};

use crate::AstFuzz;

pub(crate) trait FxFuzz: Effect
where
    AstFuzz: Distribution<Self>,
{
    fn fuzz_weight_factor() -> u32;
}

impl FxFuzz for ProcEffect {
    fn fuzz_weight_factor() -> u32 {
        1
    }
}

impl Distribution<ProcEffect> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ProcEffect {
        self.map(|()| ProcEffect::Inquire)
            .weighted_case(3)
            .or(self.map(|()| ProcEffect::Invoke).weighted_case(1))
            .sample(rng)
    }
}

impl FxFuzz for QueryEffect {
    fn fuzz_weight_factor() -> u32 {
        1
    }
}

impl Distribution<QueryEffect> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> QueryEffect {
        QueryEffect::Inquire
    }
}

impl FxFuzz for PureEffect {
    fn fuzz_weight_factor() -> u32 {
        0 // Ensure, at runtime, PureEffect are never generated by fuzzing
    }
}

impl Distribution<PureEffect> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> PureEffect {
        unreachable!("PureEffect cannot be sampled")
    }
}
