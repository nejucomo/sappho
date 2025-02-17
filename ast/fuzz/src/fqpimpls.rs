use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::{Ast, Expr};
use sappho_ast_core::{FuncDef, ObjectDef, ProcDef, QueryDef, Statements};
use sappho_attrs::IdentMap;
use sappho_object::Object;

use crate::effectsimpls::FxFuzz;
use crate::AstFuzz;

impl<FX> Distribution<ObjectDef<Ast, FX>> for AstFuzz
where
    FX: FxFuzz,
    AstFuzz: Distribution<FX>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ObjectDef<Ast, FX> {
        ObjectDef::new(
            rng.sample::<Object<FuncDef<Ast>, QueryDef<Ast>, ProcDef<Ast>, Expr<FX>>, _>(self),
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
            rng.sample::<IdentMap<A>, _>(self),
        )
    }
}

impl Distribution<FuncDef<Ast>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FuncDef<Ast> {
        FuncDef::<Ast>::new(rng.sample(self), rng.sample(self))
    }
}

impl Distribution<QueryDef<Ast>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> QueryDef<Ast> {
        QueryDef::<Ast>::new(rng.sample(self))
    }
}

impl Distribution<ProcDef<Ast>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ProcDef<Ast> {
        ProcDef::<Ast>::from(rng.sample::<Statements<Ast>, _>(self))
    }
}

impl Distribution<Statements<Ast>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Statements<Ast> {
        Statements::<Ast>::Return(self.sample(rng))
    }
}
