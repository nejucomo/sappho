//! Top-level expression type `Expr`, generic over effects [PureEffect](sappho_ast_core::PureEffect), [QueryEffect](sappho_ast_core::QueryEffect), or [ProcEffect](sappho_ast_core::ProcEffect).

use std::fmt;

use sappho_ast_core::{
    AstCore, AstTransformInto, CommentedExpr, CoreExpr, FuncDef, ObjectDef, ProcDef, QueryDef,
};
use sappho_ast_effect::Effect;
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::{Stream, Unparse};

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

impl<FX> AstTransformInto<CommentedExpr<AstCore, FX>> for Expr<FX>
where
    FX: Effect,
{
    fn ast_transform(self) -> CommentedExpr<AstCore, FX> {
        use Expr::*;

        match self {
            Core(x) => x.ast_transform(),
            Func(x) => CoreExpr::from(ObjectDef::new_func(x)).ast_transform(),
            Query(x) => CoreExpr::from(ObjectDef::new_query(x)).ast_transform(),
            Proc(x) => CoreExpr::from(ObjectDef::new_proc(x)).ast_transform(),
            List(x) => x.ast_transform(),
        }
    }
}

impl<FX> FromIterator<CommentedExpr<Ast, FX>> for Expr<FX>
where
    FX: Effect,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = CommentedExpr<Ast, FX>>,
    {
        Expr::List(ListExpr::new_from_parts(iter, None))
    }
}

impl<FX> TryIntoIdentMap<CommentedExpr<Ast, FX>> for Expr<FX>
where
    FX: Effect,
{
    fn try_into_identmap(&self) -> Option<&IdentMap<CommentedExpr<Ast, FX>>> {
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
