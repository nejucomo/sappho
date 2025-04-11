use std::fmt::Debug;

use crate::{Continuation, Step};

pub trait Eval: Sized + Debug {
    type Value: Debug;
    type Continuation: Continuation<Self::Value, Self>;

    fn eval(self) -> Self::Value {
        use Step::*;

        let mut stack: Vec<Self::Continuation> = vec![];
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

    fn eval_step(self) -> Step<Self::Value, Self, Self::Continuation>;
}
