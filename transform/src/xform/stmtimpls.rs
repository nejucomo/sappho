use sappho_ast_core::{AstProvider, Statements};
use sappho_ast_effect::ProcEffect;

use crate::xform::TransformInto;

impl<XPS, XPD> TransformInto<Statements<XPD>> for Statements<XPS>
where
    XPS: AstProvider,
    XPD: AstProvider,
    XPS::Expr<ProcEffect>: TransformInto<XPD::Expr<ProcEffect>>,
{
    fn transform(self) -> Statements<XPD> {
        use Statements::Return;

        match self {
            Return(x) => Return(Box::new(x.transform())),
        }
    }
}
