use crate::Break;

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

    pub fn write_str(&mut self, s: &str) {
        self.0.push(Leaf(s.to_string()))
    }

    pub fn write_str_break(&mut self, s: &str, brk: Break) {
        self.write_str(s);
        self.add_break(brk);
    }

    pub fn add_break(&mut self, brk: Break) {
        self.0.push(Break(brk));
    }

    pub fn add_substream(&mut self, sub: Stream) {
        self.0.push(Substream(sub));
    }
}
