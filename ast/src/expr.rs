//! Top-level expression type `GenExpr`, generic over [crate::effects]

use crate::{
    Application, CommonExpr, FuncDef, Identifier, LetExpr, ListForm, Literal, Lookup, ObjectDef,
    Pattern, PureExpr, QueryDef, QueryExpr, RecursiveExpr, UniversalExpr,
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum GenExpr<Effects> {
    Universal(UniversalExpr),
    Common(CommonExpr),
    Recursive(RecursiveExpr<Effects>),
    Effect(Effects),
}

impl<FX> GenExpr<FX> {
    pub fn num(f: f64) -> Self {
        GenExpr::Universal(UniversalExpr::Lit(Literal::Num(f)))
    }

    pub fn ref_expr(ident: Identifier) -> Self {
        GenExpr::Universal(UniversalExpr::Ref(ident))
    }

    pub fn query_expr(body: QueryExpr) -> Self {
        GenExpr::Common(CommonExpr::Query(QueryDef {
            body: Box::new(body),
        }))
    }

    pub fn func_expr((binding, body): (Pattern, PureExpr)) -> Self {
        GenExpr::Common(CommonExpr::Func(FuncDef {
            binding,
            body: Box::new(body),
        }))
    }

    pub fn object_expr(qdef: Option<QueryExpr>, fdef: Option<(Pattern, PureExpr)>) -> Self {
        GenExpr::Common(CommonExpr::Object(ObjectDef::new(
            fdef.map(|(binding, body)| FuncDef {
                binding,
                body: Box::new(body),
            }),
            qdef.map(|body| QueryDef {
                body: Box::new(body),
            }),
            sappho_identmap::IdentMap::default(),
        )))
    }

    pub fn list(exprs: Vec<Self>) -> Self {
        GenExpr::Recursive(RecursiveExpr::List(ListForm::from(exprs)))
    }

    pub fn let_expr(binding: Pattern, bindexpr: Self, tail: Self) -> Self {
        GenExpr::Recursive(RecursiveExpr::Let(LetExpr {
            binding,
            bindexpr: Box::new(bindexpr),
            tail: Box::new(tail),
        }))
    }

    pub fn application(target: Self, argument: Self) -> Self {
        GenExpr::Recursive(RecursiveExpr::Apply(Application {
            target: Box::new(target),
            argument: Box::new(argument),
        }))
    }

    pub fn lookup(target: Self, attr: Identifier) -> Self {
        GenExpr::Recursive(RecursiveExpr::Lookup(Lookup {
            target: Box::new(target),
            attr,
        }))
    }

    pub fn effect(effect: FX) -> Self {
        GenExpr::Effect(effect)
    }
}

impl<FX> fmt::Display for GenExpr<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GenExpr::*;

        match self {
            Universal(x) => x.fmt(f),
            Common(x) => x.fmt(f),
            Recursive(x) => x.fmt(f),
            Effect(x) => x.fmt(f),
        }
    }
}
