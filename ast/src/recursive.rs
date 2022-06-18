//! Recursive-subgrammars which carry their effects
//!
//! These have the same effects as the top-level expression type. For example, a list expression in
//! a pure context contains pure expressions, while a list expression in a proc context contains proc
//! expressions.

use crate::{GenExpr, Pattern};
use std::fmt;

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

impl<FX> fmt::Display for RecursiveExpr<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RecursiveExpr::*;

        match self {
            List(x) => {
                let mut first = true;
                write!(f, "[")?;
                for child in x.iter() {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    child.fmt(f)?;
                }
                write!(f, "]")?;
                Ok(())
            }
            Let(x) => x.fmt(f),
            Apply(x) => x.fmt(f),
        }
    }
}

impl<FX> fmt::Display for LetExpr<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "let ")?;
        self.binding.fmt(f)?;
        write!(f, " = ")?;
        self.bindexpr.fmt(f)?;
        write!(f, "; ")?;
        self.tail.fmt(f)?;
        Ok(())
    }
}

impl<FX> fmt::Display for Application<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        self.target.fmt(f)?;
        write!(f, " ")?;
        self.argument.fmt(f)?;
        write!(f, ")")?;
        Ok(())
    }
}
