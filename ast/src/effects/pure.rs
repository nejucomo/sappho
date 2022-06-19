use crate::GenExpr;
use std::fmt;

pub type PureExpr = GenExpr<PureEffects>;

#[derive(Debug, PartialEq)]
pub enum PureEffects {}

impl fmt::Display for PureEffects {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unreachable!("pure effects are never instantiated");
    }
}
