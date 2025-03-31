use sappho_effect::{Effect, ProcEffect, RestrictFrom};
use sappho_kast::ProcWiseParser;
use sappho_parsable::{ParsableWith, Parser};
// use sappho_syntax as syntax;
use sappho_unparse::{Stream, Unparse};

use crate::{EastProvider, ObjectDef};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<FX>
where
    FX: Effect,
{
    ObjectDef(ObjectDef<FX>),
}
