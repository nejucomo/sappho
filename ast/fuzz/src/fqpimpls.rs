use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::{Expr, FuncDef, ObjectDef, ProcDef, QueryDef, Statements};
use sappho_identmap::IdentMap;
use sappho_object::Object;

use crate::effectsimpls::FxFuzz;
use crate::AstFuzz;

impl<FX> Distribution<ObjectDef<FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ObjectDef<FX> {
        ObjectDef::new(rng.sample::<Object<FuncDef, QueryDef, ProcDef, Expr<FX>>, _>(self))
    }
}

impl<F, Q, P, A> Distribution<Object<F, Q, P, A>> for AstFuzz
where
    AstFuzz: Distribution<F> + Distribution<Q> + Distribution<P> + Distribution<A>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Object<F, Q, P, A> {
        Object::new(
            rng.sample::<Option<F>, _>(self),
            rng.sample::<Option<Q>, _>(self),
            rng.sample::<Option<P>, _>(self),
            rng.sample::<IdentMap<A>, _>(self),
        )
    }
}

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
