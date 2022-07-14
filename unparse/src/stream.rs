mod display;

use crate::{Break, Unparse};

#[derive(Debug)]
pub struct Stream {
    items: Vec<Item>,
    depth: usize,
}

#[derive(Debug)]
enum Item {
    Leaf(String),
    Break(Break),
    Substream(Stream),
}
use Item::*;

#[derive(Debug)]
pub enum Brackets {
    Parens,
    Square,
    Squiggle,
}

impl Stream {
    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn write<U>(&mut self, thing: &U)
    where
        U: Unparse,
    {
        thing.unparse_into(self)
    }

    pub fn bracketed<F>(&mut self, brackets: Brackets, f: F)
    where
        F: FnOnce(&mut Stream),
    {
        use Brackets::*;
        use Break::Opt;

        self.write(&match brackets {
            Parens => "(",
            Square => "[",
            Squiggle => "{",
        });
        self.substream(f);
        self.write(&Opt);
        self.write(&match brackets {
            Parens => ")",
            Square => "]",
            Squiggle => "}",
        });
    }

    fn substream<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Stream),
    {
        let mut sub = Stream {
            items: vec![],
            depth: self.depth + 1,
        };
        f(&mut sub);
        self.items.push(Substream(sub));
    }

    pub(crate) fn new() -> Self {
        Stream {
            items: vec![],
            depth: 0,
        }
    }

    pub(crate) fn write_string(&mut self, s: String) {
        self.items.push(Leaf(s))
    }

    pub(crate) fn add_break(&mut self, brk: Break) {
        self.items.push(Break(brk));
    }
}
