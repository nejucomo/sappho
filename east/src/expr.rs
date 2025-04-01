use sappho_effect::Effect;
use sappho_identifier::RcId;
use sappho_primval::PrimVal;

use crate::{Application, Interaction, Let, ListDef, Lookup, Match, ObjectDef};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<FX>
where
    FX: Effect,
{
    Ref(RcId),
    Prim(PrimVal),
    ObjectDef(ObjectDef<FX>),
    ListDef(ListDef<FX>),
    Let(Let<FX>),
    Match(Match<FX>),
    Application(Application<FX>),
    Lookup(Lookup<FX>),
    Interaction(Interaction<FX>),
}
