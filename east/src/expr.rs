use sappho_identifier::RcId;
use sappho_primval::PrimVal;

use crate::ObjectDef;

#[derive(Debug)]
pub enum Expr<FX> {
    Ref(RcId),
    Prim(PrimVal),
    ObjectDef(ObjectDef<FX>),
    List(List<FX>),
    Let(Let<FX>),
    Match(Match<FX>),
    Application(Application<FX>),
    Lookup(Lookup<FX>),
    Interaction(Interaction<FX>),
}
