use sappho_ast_effect::QueryEffect;
use sappho_unparse::{Stream, Unparse};

use crate::{AstProvider, AstTransformInto, BoxExpr};

/// A query definition, ie `query $x`.
#[derive(Debug, derive_new::new)]
pub struct QueryDef<XP>
where
    XP: AstProvider,
{
    /// The `QueryExpr` definition, ie the `$x` in `query $x`.
    pub body: BoxExpr<XP, QueryEffect>,
}

impl<XPS, XPD> AstTransformInto<QueryDef<XPD>> for QueryDef<XPS>
where
    XPD: AstProvider,
    XPS: AstProvider,
    XPS::Expr<QueryEffect>: AstTransformInto<XPD::Expr<QueryEffect>>,
{
    fn ast_transform(self) -> QueryDef<XPD> {
        QueryDef {
            body: self.body.ast_transform(),
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
