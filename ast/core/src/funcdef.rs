use sappho_ast_effect::PureEffect;
use sappho_unparse::{Stream, Unparse};

use crate::AstProvider;

/// A function definition expression, ie `fn x -> x`.
#[derive(Debug, derive_new::new)]
pub struct FuncDef<XP>
where
    XP: AstProvider,
{
    /// The binding pattern, ie the initial `x` in `fn x -> x`.
    pub binding: XP::Pattern,

    /// The body, ie the final `x` in `fn x -> x`.
    pub body: Box<XP::Expr<PureEffect>>,
}

impl<XP> FuncDef<XP>
where
    XP: AstProvider,
{
    pub fn transform_into<XPD>(self) -> FuncDef<XPD>
    where
        XPD: AstProvider,
        XPD::Pattern: From<XP::Pattern>,
        XPD::Expr<PureEffect>: From<XP::Expr<PureEffect>>,
    {
        FuncDef {
            binding: XPD::Pattern::from(self.binding),
            body: Box::new(XPD::Expr::from(*self.body)),
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
