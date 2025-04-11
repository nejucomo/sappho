use std::fmt::Debug;

use crate::cstack::ContinuationStack;
use crate::{Continuation, EvalStep, Scope, Step, StepContinue};

pub trait Eval<'s>: EvalStep<'s, Self> {
    type Value: Debug;
    type Locals: Default + Debug;

    fn eval(&'s self) -> Self::Value {
        self.eval_with_scope(&mut Scope::default())
    }

    fn eval_with_scope(&'s self, scope: &mut Scope<'s, Self>) -> Self::Value {
        use Step::*;

        let mut stack: ContinuationStack<'s, Self> = ContinuationStack::default();
        let mut step = self.eval_step(scope);

        loop {
            match step {
                Continue(StepContinue {
                    continuation,
                    next_expr,
                    aux,
                    locals,
                }) => {
                    if let Some(locals) = locals {
                        scope.push_locals(locals);
                    }

                    stack.push_continuation(continuation, aux);

                    step = next_expr.eval_step(&scope);
                }
                Conclude(v) => {
                    if let Some((cont, aux)) = stack.pop_continuation() {
                        step = cont.continue_with_value(v, aux);
                    } else {
                        return v;
                    }
                }
            }
        }
    }
}
