use sappho_ast_effect::PureEffect;
use sappho_unparse::{Stream, Unparse};

use crate::{CoreExpr, Pattern};

/// A function definition expression, ie `fn x -> x`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct FuncDef {
    /// The binding pattern, ie the initial `x` in `fn x -> x`.
    pub binding: Pattern,

    /// The body, ie the final `x` in `fn x -> x`.
    pub body: Box<CoreExpr<PureEffect>>,
}

impl Unparse for FuncDef {
    fn unparse_into(&self, s: &mut Stream) {
        s.write("fn ");
        s.write(&self.binding);
        s.write(" -> ");
        s.write(&self.body);
    }
}
