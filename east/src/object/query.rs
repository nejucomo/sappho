use crate::QueryExpr;
use sappho_ast as ast;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct QueryClause {
    pub body: Rc<QueryExpr>,
}

impl From<ast::QueryDef> for QueryClause {
    fn from(qd: ast::QueryDef) -> QueryClause {
        QueryClause {
            body: Rc::new(QueryExpr::from(*qd.body)),
        }
    }
}

impl fmt::Display for QueryClause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "query ")?;
        self.body.fmt(f)?;
        Ok(())
    }
}
