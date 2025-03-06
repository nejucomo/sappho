use sappho_ast as ast;
use sappho_ast_core::{AstProvider, CoreExpr};
use sappho_ast_effect::Effect;
use sappho_ast_reduced as astred;
use sappho_syntax_idstore::ArcId;

use crate::xform::TransformInto;

impl<XP, FX> TransformInto<CoreExpr<XP, FX>> for ArcId
where
    XP: AstProvider,
    FX: Effect,
{
    fn transform(self) -> CoreExpr<XP, FX> {
        CoreExpr::Ref(self)
    }
}

impl TransformInto<ast::Pattern> for ArcId {
    fn transform(self) -> ast::Pattern {
        ast::Pattern::Bind(self)
    }
}

impl TransformInto<astred::Pattern> for ArcId {
    fn transform(self) -> astred::Pattern {
        astred::Pattern::Bind(self)
    }
}
