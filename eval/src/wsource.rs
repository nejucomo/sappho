use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_value::Scope;
use sappho_with_source::WithSource;

use crate::evaltrait::Eval;
use crate::exprstep::ExprStep;

impl<'s, 'x, FX> Eval<&'s Scope, ExprStep<'x, FX>> for &'x WithSource<Expr<FX>>
where
    FX: Effect,
{
    fn eval(self, input: &'s Scope) -> ExprStep<'x, FX> {
        // TODO: When implementing user-space exception propagation, annotate the unwind/error message with source references
        self.parsed().eval(input)
    }
}
