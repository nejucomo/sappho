//! Top-level expression type `GenExpr`, generic over [crate::effects]

use crate::{
    Application, CommonExpr, FuncDef, Identifier, LetExpr, Literal, ObjectDef, Pattern, PureExpr,
    QueryDef, QueryExpr, RecursiveExpr, UniversalExpr,
};

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

    pub fn func_expr((binding, body): (Pattern, PureExpr)) -> Self {
        GenExpr::Common(CommonExpr::Func(FuncDef {
            binding,
            body: Box::new(body),
        }))
    }

    pub fn object_expr(qdef: Option<QueryExpr>, fdef: Option<(Pattern, PureExpr)>) -> Self {
        GenExpr::Common(CommonExpr::Object(ObjectDef {
            query: qdef.map(|body| QueryDef {
                body: Box::new(body),
            }),
            func: fdef.map(|(binding, body)| FuncDef {
                binding,
                body: Box::new(body),
            }),
        }))
    }

    pub fn list(exprs: Vec<Self>) -> Self {
        GenExpr::Recursive(RecursiveExpr::List(exprs))
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
}
