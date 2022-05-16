mod fromimpl;

use std::rc::Rc;

pub use saplang_ast::{Identifier, Literal, Pattern, PureEffects};

pub type PureExpr = GenExpr<PureEffects>;
pub type QueryExpr = GenExpr<QueryEffects>;
pub type ProcExpr = GenExpr<ProcEffects>;

#[derive(Debug, PartialEq)]
pub enum GenExpr<Effects> {
    Lit(Literal),
    Ref(Identifier),
    List(Vec<GenExpr<Effects>>),
    Let(LetExpr<Effects>),
    Apply(Application<Effects>),
    Object(ObjectDef),
    Effect(Effects),
}

#[derive(Debug, PartialEq)]
pub enum QueryEffects {
    Inquire(Box<QueryExpr>),
}

#[derive(Debug, PartialEq)]
pub enum ProcEffects {
    Inquire(Box<GenExpr<ProcEffects>>),
    Evoke(Box<GenExpr<ProcEffects>>),
}

#[derive(Debug, PartialEq)]
pub struct LetExpr<Effects> {
    pub binding: Pattern,
    pub bindexpr: Box<GenExpr<Effects>>,
    pub tail: Box<GenExpr<Effects>>,
}

#[derive(Debug, PartialEq)]
pub struct Application<Effects> {
    pub target: Box<GenExpr<Effects>>,
    pub argument: Box<GenExpr<Effects>>,
}

#[derive(Debug, PartialEq)]
pub struct ObjectDef {
    pub query: Option<QueryClause>,
    pub func: Option<FuncClause>,
}

#[derive(Debug, PartialEq)]
pub struct QueryClause {
    pub body: Rc<QueryExpr>,
}

#[derive(Debug, PartialEq)]
pub struct FuncClause {
    pub binding: Pattern,
    pub body: Rc<PureExpr>,
}
