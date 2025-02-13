use sappho_ast_effect::PureEffect;
use sappho_unparse::{Stream, Unparse};

use crate::{AstProvider, AstTransformInto, BoxExpr};

/// A function definition expression, ie `fn x -> x`.
#[derive(Debug, derive_new::new)]
pub struct FuncDef<XP>
where
    XP: AstProvider,
{
    /// The binding pattern, ie the initial `x` in `fn x -> x`.
    pub binding: XP::Pattern,

    /// The body, ie the final `x` in `fn x -> x`.
    pub body: BoxExpr<XP, PureEffect>,
}

impl<XPD, XPS> AstTransformInto<FuncDef<XPD>> for FuncDef<XPS>
where
    XPD: AstProvider,
    XPS: AstProvider,
    XPS::Pattern: AstTransformInto<XPD::Pattern>,
    XPS::Expr<PureEffect>: AstTransformInto<XPD::Expr<PureEffect>>,
{
    fn ast_transform(self) -> FuncDef<XPD> {
        FuncDef {
            binding: self.binding.ast_transform(),
            body: self.body.ast_transform(),
        }
    }
}

impl<XP> Unparse for FuncDef<XP>
where
    XP: AstProvider,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write("fn ");
        s.write(&self.binding);
        s.write(" -> ");
        s.write(&self.body);
    }
}

impl<XP> Clone for FuncDef<XP>
where
    XP: AstProvider,
{
    fn clone(&self) -> Self {
        FuncDef::new(self.binding.clone(), self.body.clone())
    }
}

impl<XP> PartialEq for FuncDef<XP>
where
    XP: AstProvider,
{
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.body == other.body
    }
}
