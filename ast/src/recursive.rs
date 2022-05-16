//! Recursive-subgrammars which carry their effects
//!
//! These have the same effects as the top-level expression type. For example, a list expression in
//! a pure context contains pure expressions, while a list expression in a proc context contains proc
//! expressions.
use crate::{GenExpr, Pattern};

#[derive(Debug, PartialEq)]
pub enum RecursiveExpr<Effects> {
    List(Vec<GenExpr<Effects>>),
    Let(LetExpr<Effects>),
    Apply(Application<Effects>),
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
