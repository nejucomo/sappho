use rand::distr::Distribution;
use rand::Rng;
use sappho_ast_core::{FuncDef, ObjectDef, ProcDef, QueryDef, Statements};
use sappho_ast_rich::{AstRich, Expr};
use sappho_attrs::Attrs;
use sappho_object::Object;

use crate::effectsimpls::FxFuzz;
use crate::AstFuzz;

impl<FX> Distribution<ObjectDef<AstRich, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ObjectDef<AstRich, FX> {
        ObjectDef::new(
            rng.sample::<Object<FuncDef<AstRich>, QueryDef<AstRich>, ProcDef<AstRich>, Expr<FX>>, _>(self),
        )
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
            rng.sample::<Attrs<A>, _>(self),
        )
    }
}

impl Distribution<FuncDef<AstRich>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FuncDef<AstRich> {
        FuncDef::<AstRich>::new(rng.sample(self), rng.sample(self))
    }
}

impl Distribution<QueryDef<AstRich>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> QueryDef<AstRich> {
        QueryDef::<AstRich>::new(rng.sample(self))
    }
}

impl Distribution<ProcDef<AstRich>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ProcDef<AstRich> {
        ProcDef::<AstRich>::from(rng.sample::<Statements<AstRich>, _>(self))
    }
}

impl Distribution<Statements<AstRich>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Statements<AstRich> {
        Statements::<AstRich>::Return(self.sample(rng))
    }
}
