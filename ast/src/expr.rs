//! Top-level expression type `Expr`, generic over [crate::effects]

use crate::{
    ApplicationExpr, FuncDef, Identifier, LetExpr, ListExpr, Literal, LookupExpr, MatchExpr,
    ObjectDef, QueryDef,
};
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::{Stream, Unparse};
use std::fmt;

/// The general top-level expression for all effects.
#[derive(Debug, PartialEq)]
pub enum Expr<Effects> {
    Lit(Literal),
    Ref(Identifier),
    Func(FuncDef),
    Query(QueryDef),
    Object(ObjectDef<Effects>),
    List(ListExpr<Effects>),
    Let(LetExpr<Effects>),
    Match(MatchExpr<Effects>),
    Application(ApplicationExpr<Effects>),
    Lookup(LookupExpr<Effects>),
    Effect(Effects),
}

impl<FX> From<Literal> for Expr<FX> {
    fn from(x: Literal) -> Self {
        Expr::Lit(x)
    }
}

impl<FX> From<Identifier> for Expr<FX> {
    fn from(x: Identifier) -> Self {
        Expr::Ref(x)
    }
}

impl<FX> From<FuncDef> for Expr<FX> {
    fn from(x: FuncDef) -> Self {
        Expr::Func(x)
    }
}

impl<FX> From<QueryDef> for Expr<FX> {
    fn from(x: QueryDef) -> Self {
        Expr::Query(x)
    }
}

impl<FX> From<ObjectDef<FX>> for Expr<FX> {
    fn from(x: ObjectDef<FX>) -> Self {
        Expr::Object(x)
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

impl<FX> From<LetExpr<FX>> for Expr<FX> {
    fn from(x: LetExpr<FX>) -> Self {
        Expr::Let(x)
    }
}

impl<FX> From<MatchExpr<FX>> for Expr<FX> {
    fn from(x: MatchExpr<FX>) -> Self {
        Expr::Match(x)
    }
}

impl<FX> From<ApplicationExpr<FX>> for Expr<FX> {
    fn from(x: ApplicationExpr<FX>) -> Self {
        Expr::Application(x)
    }
}

impl<FX> From<LookupExpr<FX>> for Expr<FX> {
    fn from(x: LookupExpr<FX>) -> Self {
        Expr::Lookup(x)
    }
}

impl<FX> TryIntoIdentMap<Expr<FX>> for Expr<FX> {
    fn try_into_identmap(&self) -> Option<&IdentMap<Expr<FX>>> {
        match self {
            Expr::Object(objdef) => objdef.try_into_identmap(),
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
            Lit(x) => x.unparse_into(s),
            Ref(x) => x.unparse_into(s),
            Func(x) => x.unparse_into(s),
            Query(x) => x.unparse_into(s),
            Object(x) => x.unparse_into(s),
            List(x) => x.unparse_into(s),
            Let(x) => x.unparse_into(s),
            Match(x) => x.unparse_into(s),
            Application(x) => x.unparse_into(s),
            Lookup(x) => x.unparse_into(s),
            Effect(x) => x.unparse_into(s),
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
