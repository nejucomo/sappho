use sappho_ast_effect::QueryEffect;
use sappho_unparse::{Stream, Unparse};

use crate::{AstProvider, BoxExpr};

/// A query definition, ie `query $x`.
#[derive(Debug, derive_new::new)]
pub struct QueryDef<XP>
where
    XP: AstProvider,
{
    /// The `QueryExpr` definition, ie the `$x` in `query $x`.
    pub body: BoxExpr<XP, QueryEffect>,
}

impl<XP> QueryDef<XP>
where
    XP: AstProvider,
{
    pub fn transform_into<XPD>(self) -> QueryDef<XPD>
    where
        XPD: AstProvider,
        XPD::Expr<QueryEffect>: From<XP::Expr<QueryEffect>>,
    {
        QueryDef {
            body: self.body.map_expr(XPD::Expr::from),
        }
    }
}

impl<XP> Unparse for QueryDef<XP>
where
    XP: AstProvider,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write("query ");
        s.write(&self.body);
    }
}

impl<XP> Clone for QueryDef<XP>
where
    XP: AstProvider,
{
    fn clone(&self) -> Self {
        QueryDef::new(self.body.clone())
    }
}

impl<XP> PartialEq for QueryDef<XP>
where
    XP: AstProvider,
{
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body
    }
}
