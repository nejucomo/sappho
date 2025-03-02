use sappho_ast_core::{AstProvider, FuncDef, ProcDef, QueryDef, Statements};
use sappho_ast_effect::{ProcEffect, PureEffect, QueryEffect};

use crate::xform::TransformInto;

impl<XPS, XPD> TransformInto<FuncDef<XPD>> for FuncDef<XPS>
where
    XPS: AstProvider,
    XPD: AstProvider,
    XPS::Expr<PureEffect>: TransformInto<XPD::Expr<PureEffect>>,
    XPS::Pattern: TransformInto<XPD::Pattern>,
{
    fn transform(self) -> FuncDef<XPD> {
        FuncDef::new(self.binding.transform(), Box::new(self.body.transform()))
    }
}

impl<XPS, XPD> TransformInto<QueryDef<XPD>> for QueryDef<XPS>
where
    XPS: AstProvider,
    XPD: AstProvider,
    XPS::Expr<QueryEffect>: TransformInto<XPD::Expr<QueryEffect>>,
{
    fn transform(self) -> QueryDef<XPD> {
        QueryDef::new(Box::new(self.body.transform()))
    }
}

impl<XPS, XPD> TransformInto<ProcDef<XPD>> for ProcDef<XPS>
where
    XPS: AstProvider,
    XPD: AstProvider,
    XPS::Expr<ProcEffect>: TransformInto<XPD::Expr<ProcEffect>>,
{
    fn transform(self) -> ProcDef<XPD> {
        ProcDef::from(Statements::from(self).transform())
    }
}
