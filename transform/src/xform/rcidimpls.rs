use sappho_ast_effect::Effect;
use sappho_ast_kernel::{AstProvider, CoreExpr};
use sappho_ast_red as red;
use sappho_ast_rich as rich;
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

impl TransformInto<rich::Pattern> for RcId {
    fn transform(self) -> rich::Pattern {
        rich::Pattern::Bind(self)
    }
}

impl TransformInto<red::Pattern> for RcId {
    fn transform(self) -> red::Pattern {
        red::Pattern::Bind(self)
    }
}
