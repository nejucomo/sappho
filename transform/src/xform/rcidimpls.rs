use sappho_ast as ast;
use sappho_ast_core::{AstProvider, CoreExpr};
use sappho_ast_effect::Effect;
use sappho_ast_reduced as astred;
use sappho_identifier::RcId;

use crate::xform::TransformInto;

impl<XP, FX> TransformInto<CoreExpr<XP, FX>> for RcId
where
    XP: AstProvider,
    FX: Effect,
{
    fn transform(self) -> CoreExpr<XP, FX> {
        CoreExpr::Ref(self)
    }
}

impl TransformInto<ast::Pattern> for RcId {
    fn transform(self) -> ast::Pattern {
        ast::Pattern::Bind(self)
    }
}

impl TransformInto<astred::Pattern> for RcId {
    fn transform(self) -> astred::Pattern {
        astred::Pattern::Bind(self)
    }
}
