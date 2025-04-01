use sappho_effect::Effect;
use sappho_identifier::RcId;
use sappho_primval::PrimVal;

use crate::{BoxWise, Let, ListDef, Match, ObjectDef};

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

#[derive(Clone, Debug, PartialEq)]
pub struct Application<FX>
where
    FX: Effect,
{
    target: BoxWise<FX>,
    argument: BoxWise<FX>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Lookup<FX>
where
    FX: Effect,
{
    target: BoxWise<FX>,
    attrname: RcId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Interaction<FX>
where
    FX: Effect,
{
    effect: FX,
    target: BoxWise<FX>,
}
