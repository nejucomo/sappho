use std::rc::Rc;

use derive_more::From;
use sappho_effect::Effect;
use sappho_identifier::RcId;
use sappho_primval::PrimVal;

use crate::{
    Application, FuncDef, Interaction, Let, ListDef, Lookup, Match, ObjectDef, ProcDef, QueryDef,
};

#[derive(Clone, Debug, PartialEq, From)]
pub enum Expr<FX>
where
    FX: Effect,
{
    #[from]
    Prim(PrimVal),
    /// # TODO: Define a `RefExpr` newtype
    #[from]
    Ref(RcId),
    #[from(
        ObjectDef<FX>,
        Rc<FuncDef>,
        Rc<QueryDef>,
        Rc<ProcDef>,
    )]
    ObjectDef(ObjectDef<FX>),
    #[from]
    ListDef(ListDef<FX>),
    #[from]
    Let(Let<FX>),
    #[from]
    Match(Match<FX>),
    #[from]
    Application(Application<FX>),
    #[from]
    Lookup(Lookup<FX>),
    #[from]
    Interaction(Interaction<FX>),
}
