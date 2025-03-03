use rand::distr::{Distribution, StandardUniform};
use rand::Rng;
use sappho_ast_core::Literal;
use sappho_identifier::{Identifier, RcId};

use crate::AstFuzz;

impl Distribution<Literal> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Literal {
        let f: f64 = if rng.random_ratio(4, 5) {
            // 4 of 5 numbers are an integer:
            let range = if rng.random_ratio(2, 3) {
                // 2 out of 3 integers is 0 or 1:
                0..=1
            } else {
                // The other integers are in this range:
                -99..=99
            };

            let i: i32 = rng.random_range(range);
            f64::from(i)
        } else {
            // The rest are random floats:
            self.sample(rng)
        };
        Literal::Num(f)
    }
}

impl Distribution<RcId> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RcId {
        RcId::from(rng.sample::<Identifier, _>(self))
    }
}

impl Distribution<Identifier> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Identifier {
        let mut id = "".to_string();
        while id.is_empty() || rng.random_ratio(1, 3) {
            id.push(rng.random_range('a'..='z'));
        }
        Identifier::new(id).unwrap()
    }
}

impl Distribution<f64> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        StandardUniform.sample(rng)
    }
}

impl Distribution<()> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) {}
}
