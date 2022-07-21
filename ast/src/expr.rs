//! Top-level expression type `Expr`, generic over [crate::effects]

use crate::{CoreExpr, FuncDef, ListExpr, ProcDef, QueryDef};
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::{Stream, Unparse};
use std::fmt;

/// The general top-level expression for all effects.
#[derive(Debug, PartialEq)]
pub enum Expr<Effects> {
    Core(CoreExpr<Effects>),

    // Extensions from Core:
    Func(FuncDef),
    Query(QueryDef),
    Proc(ProcDef),
    List(ListExpr<Effects>),
}

impl<FX, T> From<T> for Expr<FX>
where
    CoreExpr<FX>: From<T>,
{
    fn from(x: T) -> Self {
        Expr::Core(CoreExpr::from(x))
    }
}

impl<FX> FromIterator<Expr<FX>> for Expr<FX> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Expr<FX>>,
    {
        Expr::List(ListExpr::new(iter, None))
    }
}

impl<FX> TryIntoIdentMap<Expr<FX>> for Expr<FX> {
    fn try_into_identmap(&self) -> Option<&IdentMap<Expr<FX>>> {
        match self {
            Expr::Core(c) => c.try_into_identmap(),
            _ => None,
        }
    }
}

impl<FX> Unparse for Expr<FX>
where
    FX: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use Expr::*;

        match self {
            Core(x) => x.unparse_into(s),
            Func(x) => x.unparse_into(s),
            Query(x) => x.unparse_into(s),
            Proc(x) => x.unparse_into(s),
            List(x) => x.unparse_into(s),
        }
    }
}

impl<FX> fmt::Display for Expr<FX>
where
    FX: Unparse,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unparse().fmt(f)
    }
}
