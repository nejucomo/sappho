use std::error::Error;

use derive_more::Deref;
use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_value::{Locals, Scope, Value};

use crate::continuation::EvalNext;
use crate::evalstep::EvalStep;
use crate::expr::ContExpr;
use crate::step::{ContinueStep, Step::*};
use crate::withlocals::WithLocals;

pub type EvalResult<T> = Result<T, EvalError>;
pub type EvalError = Box<dyn Error + 'static>;

pub fn eval<'s, FX>(expr: &'s Expr<FX>) -> EvalResult<Value>
where
    FX: Effect,
{
    eval_in_scope(expr, Scope::default())
}

pub fn eval_in_scope<'s, FX>(expr: &'s Expr<FX>, scope: Scope) -> EvalResult<Value>
where
    FX: Effect,
{
    let mut scopeslot = ScopeSlot(scope);
    let mut cstack = vec![];
    let mut step: EvalStep<'s, FX> = expr.eval_next(&scopeslot);

    loop {
        match step {
            Produce(res) => {
                // FIXME: replace this short circuit err escalation:
                let v = res?;

                if let Some(StackItem { cont, pop_scope }) = cstack.pop() {
                    scopeslot.pop_locals_if(pop_scope);
                    step = cont.continue_with(v);
                } else {
                    return Ok(v);
                }
            }
            Continue(ContinueStep {
                expr: WithLocals { expr, optlocals },
                cont: c,
            }) => {
                cstack.push(StackItem {
                    cont: c,
                    pop_scope: scopeslot.push_locals_opt(optlocals),
                });

                step = expr.eval_next(&scopeslot);
            }
        }
    }
}

#[derive(Debug)]
struct StackItem<'s, FX> {
    cont: ContExpr<'s, FX>,
    pop_scope: bool,
}

#[derive(Debug, Deref)]
struct ScopeSlot(Scope);

impl ScopeSlot {
    fn push_locals_opt(&mut self, optlocals: Option<Locals>) -> bool {
        if let Some(locals) = optlocals {
            self.0 = self.0.push_locals(locals);
            true
        } else {
            false
        }
    }

    fn pop_locals_if(&mut self, do_pop: bool) {
        if do_pop {
            self.pop_locals()
        }
    }

    fn pop_locals(&mut self) {
        self.0 = self
            .0
            .pop_locals()
            .map(|(s, _)| s)
            .expect("cstack underrun");
    }
}
