use sappho_ast_core::{AstProvider, CoreExpr};
use sappho_ast_effect::Effect;
use sappho_ast_reduced as red;
use sappho_ast_rich as rich;
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

impl TransformInto<rich::Pattern> for ArcId {
    fn transform(self) -> rich::Pattern {
        rich::Pattern::Bind(self)
    }
}

impl TransformInto<red::Pattern> for ArcId {
    fn transform(self) -> red::Pattern {
        red::Pattern::Bind(self)
    }
}
