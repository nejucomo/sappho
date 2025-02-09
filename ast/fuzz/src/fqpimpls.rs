use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::{ProcDef, QueryDef, Statements};

use crate::AstFuzz;

impl Distribution<QueryDef> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> QueryDef {
        QueryDef::new(rng.sample(self))
    }
}

impl Distribution<ProcDef> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ProcDef {
        ProcDef::from(rng.sample::<Statements, _>(self))
    }
}

impl Distribution<Statements> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Statements {
        Statements::Return(self.sample(rng))
    }
}
