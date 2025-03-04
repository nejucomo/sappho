use sappho_ast_core::{AstProvider, CmtExpr};
use sappho_ast_effect::Effect;
use sappho_unparse::Unparse;

use crate::xform::TransformInto;

impl<XPS, XPD, FX> TransformInto<CmtExpr<XPD, FX>> for CmtExpr<XPS, FX>
where
    XPS: AstProvider,
    XPD: AstProvider,
    FX: Effect,
    XPS::Expr<FX>: TransformInto<XPD::Expr<FX>>,
{
    fn transform(self) -> CmtExpr<XPD, FX> {
        let src = self.expr().unparse().to_string();
        self.append_comment_section("Transformed From", src)
            .map_expr(XPS::Expr::transform)
    }
}
