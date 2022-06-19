use crate::{Pattern, PureExpr};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct FuncDef {
    pub binding: Pattern,
    pub body: Box<PureExpr>,
}

impl fmt::Display for FuncDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fn ")?;
        self.binding.fmt(f)?;
        write!(f, " -> ")?;
        self.body.fmt(f)?;
        Ok(())
    }
}
