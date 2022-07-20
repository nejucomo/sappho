use crate::{EvalV, Result};
use sappho_ast_reduced::Literal;
use sappho_value::{ScopeRef, Value};

impl EvalV for Literal {
    fn eval_val(&self, _scope: &ScopeRef) -> Result<Value> {
        Ok(match self {
            Literal::Num(f) => Value::Num(*f),
        })
    }
}
