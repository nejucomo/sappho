use sappho_effect::PureEffect;

use crate::evfx::{ContinuationStack, HasCStack};
use crate::expr::ExprCont;

impl HasCStack for PureEffect {
    type Stack = Vec<ExprCont<PureEffect>>;
    type TransitionalCont = ExprCont<Self>;
}

impl ContinuationStack<PureEffect> for Vec<ExprCont<PureEffect>> {
    fn push(&mut self, txc: ExprCont<PureEffect>) {
        Vec::push(self, txc)
    }

    fn pop(&mut self) -> Option<ExprCont<PureEffect>> {
        Vec::pop(self)
    }
}
