//! "effects" are pure, query, or proc
//!
//! - pure: deterministic incomplete functions
//! - query: expressions that only read mutable values
//! - proc: mutate state

use crate::GenExpr;
use std::fmt;

pub type PureExpr = GenExpr<PureEffects>;
pub type QueryExpr = GenExpr<QueryEffects>;
pub type ProcExpr = GenExpr<ProcEffects>;

#[derive(Debug, PartialEq)]
pub enum PureEffects {}

#[derive(Debug, PartialEq)]
pub enum QueryEffects {
    Inquire(Box<GenExpr<QueryEffects>>),
}

#[derive(Debug, PartialEq)]
pub enum ProcEffects {
    Inquire(Box<GenExpr<ProcEffects>>),
    Evoke(Box<GenExpr<ProcEffects>>),
}

impl fmt::Display for PureEffects {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unreachable!("pure effects are never instantiated");
    }
}

impl fmt::Display for QueryEffects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use QueryEffects::*;

        match self {
            Inquire(x) => {
                write!(f, "$")?;
                x.fmt(f)
            }
        }
    }
}

impl fmt::Display for ProcEffects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ProcEffects::*;

        match self {
            Inquire(x) => {
                write!(f, "$")?;
                x.fmt(f)
            }
            Evoke(x) => {
                write!(f, "!")?;
                x.fmt(f)
            }
        }
    }
}
