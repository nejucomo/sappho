mod display;

use crate::{Break, Unparse};

#[derive(Debug)]
pub struct Stream(Vec<Item>);

#[derive(Debug)]
enum Item {
    Leaf(String),
    Break(Break),
    Substream(Stream),
}
use Item::*;

impl Stream {
    pub fn write<U>(&mut self, thing: &U)
    where
        U: Unparse,
    {
        thing.unparse_into(self)
    }

    pub fn substream<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Stream),
    {
        let mut sub = Stream::new();
        f(&mut sub);
        self.0.push(Substream(sub));
    }

    pub(crate) fn new() -> Self {
        Stream(vec![])
    }

    pub(crate) fn write_string(&mut self, s: String) {
        self.0.push(Leaf(s))
    }

    pub(crate) fn add_break(&mut self, brk: Break) {
        self.0.push(Break(brk));
    }
}
