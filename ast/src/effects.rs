//! "effects" are pure, query, or proc
//!
//! - pure: deterministic incomplete functions
//! - query: expressions that only read mutable values
//! - proc: mutate state
use crate::GenExpr;

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
