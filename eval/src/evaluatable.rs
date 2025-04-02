use sappho_value::Value;

pub trait Evaluatable {
    fn eval(&self) -> Value;
}
