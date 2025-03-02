use sappho_ast_core::{AstProvider, FuncDef, ObjectDef, ProcDef, QueryDef};
use sappho_ast_effect::Effect;
use sappho_object::Object;

use crate::xform::TransformInto;

impl<XPS, XPD, FX> TransformInto<ObjectDef<XPD, FX>> for ObjectDef<XPS, FX>
where
    XPS: AstProvider,
    XPD: AstProvider,
    FX: Effect,
    XPS::Expr<FX>: TransformInto<XPD::Expr<FX>>,
    FuncDef<XPS>: TransformInto<FuncDef<XPD>>,
    QueryDef<XPS>: TransformInto<QueryDef<XPD>>,
    ProcDef<XPS>: TransformInto<ProcDef<XPD>>,
{
    fn transform(self) -> ObjectDef<XPD, FX> {
        ObjectDef::from(Object::from(self).transform())
    }
}

impl<SF, SQ, SP, SA, DF, DQ, DP, DA> TransformInto<Object<DF, DQ, DP, DA>>
    for Object<SF, SQ, SP, SA>
where
    SF: TransformInto<DF>,
    SQ: TransformInto<DQ>,
    SP: TransformInto<DP>,
    SA: TransformInto<DA>,
{
    fn transform(self) -> Object<DF, DQ, DP, DA> {
        self.map_parts(SF::transform, SQ::transform, SP::transform, SA::transform)
    }
}
