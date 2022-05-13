use crate::Result;
use derive_more::From;
use std::rc::Rc;

pub type ValRef = Rc<Value>;

#[derive(Debug, From)]
pub enum Value {
    Num(f64),
    List(List),
    Object(Object),
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

pub struct Object {
    pub func: Option<Box<dyn Fn(ValRef) -> Result<ValRef>>>,
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "<object{}>",
            if self.func.is_some() { " fn" } else { "" }
        )
    }
}
