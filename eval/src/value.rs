use std::rc::Rc;

pub type ValRef = Rc<Value>;

#[derive(Debug, PartialEq)]
pub enum Value {
    Num(f64),
}
