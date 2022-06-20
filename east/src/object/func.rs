use crate::{Pattern, PureExpr};
use sappho_ast as ast;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct FuncClause {
    pub binding: Pattern,
    pub body: Rc<PureExpr>,
}

impl From<ast::FuncDef> for FuncClause {
    fn from(fd: ast::FuncDef) -> FuncClause {
        FuncClause {
            binding: fd.binding,
            body: Rc::new(PureExpr::from(*fd.body)),
        }
    }
}

impl fmt::Display for FuncClause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fn ")?;
        self.binding.fmt(f)?;
        write!(f, " -> ")?;
        self.body.fmt(f)?;
        Ok(())
    }
}
