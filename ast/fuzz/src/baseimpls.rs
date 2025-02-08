use rand::distr::{Distribution, StandardUniform};
use rand::Rng;
use sappho_ast::{Identifier, Literal};

use crate::{AstFuzz, DistributionExt};

impl Distribution<Literal> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Literal {
        self.weighted_case(1, Literal::Num).sample(rng)
    }
}

impl Distribution<Identifier> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Identifier {
        let mut id = "".to_string();
        while id.is_empty() || rng.random_ratio(2, 3) {
            id.push(rng.random_range('a'..='z'));
        }
        id
    }
}

impl Distribution<f64> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        StandardUniform.sample(rng)
    }
}
