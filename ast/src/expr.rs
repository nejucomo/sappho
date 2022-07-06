//! Top-level expression type `GenExpr`, generic over [crate::effects]

use crate::{
    ApplicationExpr, FuncDef, Identifier, LetExpr, Literal, LookupExpr, MatchExpr, ObjectDef,
    QueryDef,
};
use sappho_gast::ListForm;
use std::fmt;

/// The general top-level expression for all effects.
#[derive(Debug, PartialEq)]
pub enum GenExpr<Effects> {
    Lit(Literal),
    Ref(Identifier),
    Func(FuncDef),
    Query(QueryDef),
    Object(ObjectDef<Effects>),
    List(ListForm<GenExpr<Effects>>),
    Let(LetExpr<Effects>),
    Match(MatchExpr<Effects>),
    Application(ApplicationExpr<Effects>),
    Lookup(LookupExpr<Effects>),
    Effect(Effects),
}

impl<FX> From<Literal> for GenExpr<FX> {
    fn from(x: Literal) -> Self {
        GenExpr::Lit(x)
    }
}

impl<FX> From<Identifier> for GenExpr<FX> {
    fn from(x: Identifier) -> Self {
        GenExpr::Ref(x)
    }
}

impl<FX> From<FuncDef> for GenExpr<FX> {
    fn from(x: FuncDef) -> Self {
        GenExpr::Func(x)
    }
}

impl<FX> From<QueryDef> for GenExpr<FX> {
    fn from(x: QueryDef) -> Self {
        GenExpr::Query(x)
    }
}

impl<FX> From<ObjectDef<FX>> for GenExpr<FX> {
    fn from(x: ObjectDef<FX>) -> Self {
        GenExpr::Object(x)
    }
}

impl<FX> FromIterator<GenExpr<FX>> for GenExpr<FX> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = GenExpr<FX>>,
    {
        GenExpr::List(iter.into_iter().collect())
    }
}

impl<FX> From<LetExpr<FX>> for GenExpr<FX> {
    fn from(x: LetExpr<FX>) -> Self {
        GenExpr::Let(x)
    }
}

impl<FX> From<MatchExpr<FX>> for GenExpr<FX> {
    fn from(x: MatchExpr<FX>) -> Self {
        GenExpr::Match(x)
    }
}

impl<FX> From<ApplicationExpr<FX>> for GenExpr<FX> {
    fn from(x: ApplicationExpr<FX>) -> Self {
        GenExpr::Application(x)
    }
}

impl<FX> From<LookupExpr<FX>> for GenExpr<FX> {
    fn from(x: LookupExpr<FX>) -> Self {
        GenExpr::Lookup(x)
    }
}

impl<FX> fmt::Display for GenExpr<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GenExpr::*;

        match self {
            Lit(x) => x.fmt(f),
            Ref(x) => x.fmt(f),
            Func(x) => x.fmt(f),
            Query(x) => x.fmt(f),
            Object(x) => x.fmt(f),
            List(x) => x.fmt(f),
            Let(x) => x.fmt(f),
            Match(x) => x.fmt(f),
            Application(x) => x.fmt(f),
            Lookup(x) => x.fmt(f),
            Effect(x) => x.fmt(f),
        }
    }
}
