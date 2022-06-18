use crate::ValRef;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug)]
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

pub struct ListIter(Rc<List>);

impl IntoIterator for List {
    type IntoIter = ListIter;
    type Item = ValRef;

    fn into_iter(self) -> Self::IntoIter {
        ListIter(Rc::new(self))
    }
}

impl Iterator for ListIter {
    type Item = ValRef;

    fn next(&mut self) -> Option<Self::Item> {
        use List::*;

        if let Some((v, tail)) = match self.0.as_ref() {
            Empty => None,
            Cell(v, tail) => Some((v.clone(), tail.clone())),
        } {
            self.0 = tail;
            Some(v)
        } else {
            None
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        write!(f, "[")?;
        for v in self.clone() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            v.fmt(f)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}
