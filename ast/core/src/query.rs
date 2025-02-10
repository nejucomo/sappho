use sappho_ast_effect::QueryEffect;
use sappho_unparse::{Stream, Unparse};

use crate::ExprProvider;

/// A query definition, ie `query $x`.
#[derive(Clone, Debug, derive_new::new)]
pub struct QueryDef<XP>
where
    XP: ExprProvider,
{
    /// The `QueryExpr` definition, ie the `$x` in `query $x`.
    pub body: Box<XP::Expr<QueryEffect>>,
}

impl<XP> QueryDef<XP>
where
    XP: ExprProvider,
{
    pub fn transform_into<XPD>(self) -> QueryDef<XPD>
    where
        XPD: ExprProvider,
        XPD::Expr<QueryEffect>: From<XP::Expr<QueryEffect>>,
    {
        QueryDef {
            body: Box::new(XPD::Expr::from(*self.body)),
        }
    }
}

impl<XP> Unparse for QueryDef<XP>
where
    XP: ExprProvider,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write("query ");
        s.write(&self.body);
    }
}

impl<XP> PartialEq for QueryDef<XP>
where
    XP: ExprProvider,
{
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body
    }
}
