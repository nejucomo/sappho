use crate::GenExpr;
use std::fmt;

pub type QueryExpr = GenExpr<QueryEffects>;

#[derive(Debug, PartialEq)]
pub enum QueryEffects {
    Inquire(Box<GenExpr<QueryEffects>>),
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
