use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::{Pattern, UnpackPattern};
use sappho_identmap::IdentMap;
use sappho_rand_dcomp::{DistributionExt, WeightedCase};

use crate::AstFuzz;

impl Distribution<Pattern> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Pattern {
        self.map(Pattern::Bind)
            .weighted_case(1)
            .or(self.map(Pattern::LitEq).weighted_case(1))
            .or(self.map(Pattern::Unpack).weighted_case(1))
            .or(self.map(Pattern::List).weighted_case(1))
            .sample(rng)
    }
}

impl Distribution<UnpackPattern> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UnpackPattern {
        UnpackPattern::from(rng.sample::<IdentMap<_>, _>(self))
    }
}
