//! Top-level expression type `GenExpr`, generic over [crate::effects]

use crate::{
    ApplicationExpr, FuncDef, Identifier, LetExpr, ListForm, Literal, Lookup, ObjectDef, Pattern,
    PureExpr, QueryDef, QueryExpr,
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum GenExpr<Effects> {
    Lit(Literal),
    Ref(Identifier),
    Func(FuncDef),
    Query(QueryDef),
    Object(ObjectDef),
    List(ListForm<GenExpr<Effects>>),
    Let(LetExpr<Effects>),
    Application(ApplicationExpr<Effects>),
    Lookup(Lookup<Effects>),
    Effect(Effects),
}

impl<FX> GenExpr<FX> {
    pub fn num(f: f64) -> Self {
        GenExpr::Lit(Literal::Num(f))
    }

    pub fn ref_expr(ident: Identifier) -> Self {
        GenExpr::Ref(ident)
    }

    pub fn query_expr(body: QueryExpr) -> Self {
        GenExpr::Query(QueryDef {
            body: Box::new(body),
        })
    }

    pub fn func_expr((binding, body): (Pattern, PureExpr)) -> Self {
        GenExpr::Func(FuncDef {
            binding,
            body: Box::new(body),
        })
    }

    pub fn object_expr(qdef: Option<QueryExpr>, fdef: Option<(Pattern, PureExpr)>) -> Self {
        GenExpr::Object(ObjectDef::new(
            fdef.map(|(binding, body)| FuncDef {
                binding,
                body: Box::new(body),
            }),
            qdef.map(|body| QueryDef {
                body: Box::new(body),
            }),
            sappho_identmap::IdentMap::default(),
        ))
    }

    pub fn list(exprs: Vec<Self>) -> Self {
        GenExpr::List(ListForm::from(exprs))
    }

    pub fn let_expr(binding: Pattern, bindexpr: Self, tail: Self) -> Self {
        GenExpr::Let(LetExpr {
            binding,
            bindexpr: Box::new(bindexpr),
            tail: Box::new(tail),
        })
    }

    pub fn application(target: Self, argument: Self) -> Self {
        GenExpr::Application(ApplicationExpr {
            target: Box::new(target),
            argument: Box::new(argument),
        })
    }

    pub fn lookup(target: Self, attr: Identifier) -> Self {
        GenExpr::Lookup(Lookup {
            target: Box::new(target),
            attr,
        })
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
            Lit(x) => x.fmt(f),
            Ref(x) => x.fmt(f),
            Func(x) => x.fmt(f),
            Query(x) => x.fmt(f),
            Object(x) => x.fmt(f),
            List(x) => x.fmt(f),
            Let(x) => x.fmt(f),
            Application(x) => x.fmt(f),
            Lookup(x) => x.fmt(f),
            Effect(x) => x.fmt(f),
        }
    }
}
