use std::fmt::Debug;

use crate::{Continuation, EvalStep, Step};

pub trait Eval<V, C>: EvalStep<V, Self, C>
where
    V: Debug,
    C: Continuation<V, Self>,
{
    fn eval(self) -> V {
        use Step::*;

        let mut stack: Vec<C> = vec![];
        let mut step = self.eval_step();

        loop {
            match step {
                Continue(x, c) => {
                    stack.push(c);

                    step = x.eval_step();
                }
                Conclude(v) => {
                    if let Some(c) = stack.pop() {
                        step = c.continue_with_value(v);
                    } else {
                        return v;
                    }
                }
            }
        }
    }
}
