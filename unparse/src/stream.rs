use crate::{Break, Unparse};
use std::fmt;

#[derive(Debug, Default)]
pub struct Stream(Vec<Item>);

#[derive(Debug)]
enum Item {
    Leaf(String),
    Break(Break),
    Substream(Stream),
}
use Item::*;

impl Stream {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write<U>(&mut self, thing: &U)
    where
        U: Unparse,
    {
        thing.unparse_into(self)
    }

    pub fn write_string(&mut self, s: String) {
        self.0.push(Leaf(s))
    }

    pub fn add_break(&mut self, brk: Break) {
        self.0.push(Break(brk));
    }

    pub fn add_substream(&mut self, sub: Stream) {
        self.0.push(Substream(sub));
    }
}

impl fmt::Display for Stream {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        todo!();
    }
}
