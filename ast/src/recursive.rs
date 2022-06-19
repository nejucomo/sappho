//! Recursive-subgrammars which carry their effects
//!
//! These have the same effects as the top-level expression type. For example, a list expression in
//! a pure context contains pure expressions, while a list expression in a proc context contains proc
//! expressions.

mod apply;
mod letexpr;
mod lookup;

use crate::GenExpr;
use std::fmt;

pub use self::apply::Application;
pub use self::letexpr::LetExpr;
pub use self::lookup::Lookup;

#[derive(Debug, PartialEq)]
pub enum RecursiveExpr<Effects> {
    List(Vec<GenExpr<Effects>>),
    Let(LetExpr<Effects>),
    Apply(Application<Effects>),
    Lookup(Lookup<Effects>),
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
            Lookup(x) => x.fmt(f),
        }
    }
}
