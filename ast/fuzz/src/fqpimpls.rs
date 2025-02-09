use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::{FuncDef, ProcDef, QueryDef, Statements};

use crate::AstFuzz;

impl Distribution<FuncDef> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FuncDef {
        FuncDef::new(rng.sample(self), rng.sample(self))
    }
}

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
