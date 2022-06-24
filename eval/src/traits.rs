use crate::Result;
use sappho_value::{ScopeRef, ValRef, Value};

pub(crate) fn trace_eval<T>(x: &T, scope: &ScopeRef) -> Result<ValRef>
where
    T: Eval + std::fmt::Display,
{
    log::debug!("Evaluating:\n  From {}\n  ...", x);
    let r = x.eval(scope);
    log::debug!(
        "Evaluated:\n  From: {}\n  To: {}\n",
        x,
        match &r {
            Ok(v) => v.to_string(),
            Err(e) => format!("{:?}", e),
        }
    );
    r
}

pub(crate) trait Eval {
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef>;
}

pub(crate) trait EvalV {
    fn eval_val(&self, scope: &ScopeRef) -> Result<Value>;
}

pub(crate) trait EvalThunk {
    fn eval_thunk(&self) -> Result<ValRef>;
}

impl<T> Eval for T
where
    T: EvalV,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        self.eval_val(scope).map(ValRef::from)
    }
}
