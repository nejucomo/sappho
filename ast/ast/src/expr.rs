//! Top-level expression type `Expr`, generic over effects [PureEffect](sappho_ast_core::PureEffect), [QueryEffect](sappho_ast_core::QueryEffect), or [ProcEffect](sappho_ast_core::ProcEffect).

use either::Either;
use sappho_ast_core::{CoreExpr, FuncDef, ProcDef, QueryDef};
use sappho_ast_effect::Effect;
use sappho_attrs::{Attrs, TryIntoAttrs};
use sappho_unparse::{Stream, Unparse};
use std::fmt;

use crate::{Ast, ListExpr};

/// The general top-level expression for all effects.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr<FX>
where
    FX: Effect,
{
    Core(CoreExpr<Ast, FX>),

    // Extensions from Core:
    Func(FuncDef<Ast>),
    Query(QueryDef<Ast>),
    Proc(ProcDef<Ast>),
    List(ListExpr<FX>),
}

impl<FX, T> From<T> for Expr<FX>
where
    FX: Effect,
    CoreExpr<Ast, FX>: From<T>,
{
    fn from(x: T) -> Self {
        Expr::Core(CoreExpr::from(x))
    }
}

impl<FX> FromIterator<Either<Expr<FX>, Box<Expr<FX>>>> for Expr<FX>
where
    FX: Effect,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Either<Expr<FX>, Box<Expr<FX>>>>,
    {
        Expr::List(ListExpr::from_iter(iter))
    }
}

impl<FX> TryIntoAttrs<Expr<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn try_into_identmap(&self) -> Option<&Attrs<Expr<FX>>> {
        match self {
            Expr::Core(c) => c.try_into_identmap(),
            _ => None,
        }
    }
}

impl<FX> Unparse for Expr<FX>
where
    FX: Effect,
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
    FX: Effect,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unparse().fmt(f)
    }
}
