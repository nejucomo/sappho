use derive_more::From;
use sappho_east as east;
use sappho_effect::Effect;
use sappho_primval::PrimVal;

use crate::scian::ScopeAnnotated;
use crate::{AnnotateScope, AnnotationError, IxRef, ScopeInfoBuilder};

#[derive(Clone, Debug, PartialEq, From)]
pub enum Expr<FX>
where
    FX: Effect,
{
    Prim(PrimVal),
    Ref(IxRef),
    // ObjectDef(ObjectDef<FX>),
    // ListDef(ListDef<FX>),
    // Let(Let<FX>),
    // Match(Match<FX>),
    // Application(Application<FX>),
    // Lookup(Lookup<FX>),
    // Interaction(Interaction<FX>),
    Todo(std::marker::PhantomData<FX>),
}

impl<FX> AnnotateScope for east::Expr<FX>
where
    FX: Effect,
{
    type BeastNode = Expr<FX>;

    fn asi_recurse(self, scib: &mut ScopeInfoBuilder) -> Result<Self::BeastNode, AnnotationError> {
        use east::Expr::*;

        match self {
            Prim(x) => Ok(Expr::Prim(x)),
            Ref(rcid) => scib.register_idref(rcid).map(Expr::Ref),

            // ObjectDef(x) => fixme,
            // ListDef(x) => fixme,
            // Let(x) => fixme,
            // Match(x) => fixme,
            // Application(x) => fixme,
            // Lookup(x) => fixme,
            // Interaction(x) => fixme,
            other => todo!("{other:?}"),
        }
    }
}
