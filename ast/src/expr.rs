//! Top-level expression type `GenExpr`, generic over [crate::effects]

use crate::{
    ApplicationExpr, FuncDef, Identifier, LetExpr, ListExpr, Literal, LookupExpr, MatchExpr,
    ObjectDef, QueryDef,
};
use sappho_fmtutil::{DisplayDepth, FmtResult, Formatter};
use sappho_identmap::{IdentMap, TryIntoIdentMap};

/// The general top-level expression for all effects.
#[derive(Debug, PartialEq)]
pub enum GenExpr<Effects> {
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
        GenExpr::List(ListExpr::new(iter, None))
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

impl<FX> TryIntoIdentMap<GenExpr<FX>> for GenExpr<FX> {
    fn try_into_identmap(&self) -> Option<&IdentMap<GenExpr<FX>>> {
        match self {
            GenExpr::Object(objdef) => objdef.try_into_identmap(),
            _ => None,
        }
    }
}

impl<FX> DisplayDepth for GenExpr<FX>
where
    FX: DisplayDepth,
{
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> FmtResult {
        use GenExpr::*;

        match self {
            Lit(x) => x.fmt_depth(f, depth),
            Ref(x) => x.fmt_depth(f, depth),
            Func(x) => x.fmt_depth(f, depth),
            Query(x) => x.fmt_depth(f, depth),
            Object(x) => x.fmt_depth(f, depth),
            List(x) => x.fmt_depth(f, depth),
            Let(x) => x.fmt_depth(f, depth),
            Match(x) => x.fmt_depth(f, depth),
            Application(x) => x.fmt_depth(f, depth),
            Lookup(x) => x.fmt_depth(f, depth),
            Effect(x) => x.fmt_depth(f, depth),
        }
    }
}

impl<FX> std::fmt::Display for GenExpr<FX>
where
    FX: DisplayDepth,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.fmt_depth(f, 0)
    }
}
