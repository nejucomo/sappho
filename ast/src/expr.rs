//! Top-level expression type `Expr`, generic over effects [PureEffects](sappho_ast_core::PureEffects), [QueryEffects](sappho_ast_core::QueryEffects), or [ProcEffects](sappho_ast_core::ProcEffects).

use crate::{CoreExpr, FuncDef, ListExpr, ProcDef, QueryDef};
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_legible::{IntoNode, Legible, Node};
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

impl<'a, FX> IntoNode for &'a Expr<FX>
where
    &'a FX: IntoNode,
{
    fn into_node(self) -> Node {
        use Expr::*;

        match self {
            Core(x) => x.into_node(),
            Func(x) => x.into_node(),
            Query(x) => x.into_node(),
            Proc(x) => x.into_node(),
            List(x) => x.into_node(),
        }
    }
}

impl<FX> fmt::Display for Expr<FX>
where
    for<'a> &'a FX: IntoNode,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_legible(f)
    }
}
