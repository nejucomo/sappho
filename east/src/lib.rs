mod fromimpl;

use std::rc::Rc;

pub use saplang_ast::{Identifier, Literal, Pattern};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Lit(Literal),
    Ref(Identifier),
    List(Vec<Expr>),
    Let(LetExpr),
    Apply(Application),
    Object(ObjectExpr),
}

#[derive(Debug, PartialEq)]
pub struct LetExpr {
    pub binding: Pattern,
    pub bindexpr: Box<Expr>,
    pub tail: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Application {
    pub target: Box<Expr>,
    pub argument: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct ObjectExpr {
    pub func: Option<FuncClause>,
}

#[derive(Debug, PartialEq)]
pub struct FuncClause {
    pub binding: Pattern,
    pub body: Rc<Expr>,
}
