use sappho_ast_effect::PureEffect;
use sappho_unparse::{Stream, Unparse};

use crate::ExprProvider;

/// A function definition expression, ie `fn x -> x`.
#[derive(Clone, Debug, derive_new::new)]
pub struct FuncDef<XP>
where
    XP: ExprProvider,
{
    /// The binding pattern, ie the initial `x` in `fn x -> x`.
    pub binding: XP::Pattern,

    /// The body, ie the final `x` in `fn x -> x`.
    pub body: Box<XP::Expr<PureEffect>>,
}

impl<XP> FuncDef<XP>
where
    XP: ExprProvider,
{
    pub fn transform_into<XPD>(self) -> FuncDef<XPD>
    where
        XPD: ExprProvider,
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
    XP: ExprProvider,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write("fn ");
        s.write(&self.binding);
        s.write(" -> ");
        s.write(&self.body);
    }
}

impl<XP> PartialEq for FuncDef<XP>
where
    XP: ExprProvider,
{
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.body == other.body
    }
}
