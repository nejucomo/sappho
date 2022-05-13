use crate::Result;
use derive_more::From;
use std::rc::Rc;

pub type ValRef = Rc<Value>;

#[derive(Debug, From)]
pub enum Value {
    Num(f64),
    List(List),
    Func(FuncObj),
}

#[derive(Debug)]
pub enum List {
    Empty,
    Cell(ValRef, Rc<List>),
}

impl Default for List {
    fn default() -> List {
        List::Empty
    }
}

impl List {
    pub fn prepend(self, vref: ValRef) -> List {
        List::Cell(vref, Rc::new(self))
    }
}

pub struct FuncObj(pub Box<dyn Fn(ValRef) -> Result<ValRef>>);

impl std::fmt::Debug for FuncObj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<func>")
    }
}
