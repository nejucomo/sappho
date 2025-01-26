use std::ops::Deref;

use crate::{Result, Stream};

pub trait Unparse {
    fn unparse<'a, 'b>(&self, stream: &mut Stream<'a, 'b>) -> Result<()>;
}

impl Unparse for str {
    fn unparse<'a, 'b>(&self, stream: &mut Stream<'a, 'b>) -> Result<()> {
        stream.write_str(self)
    }
}

impl Unparse for char {
    fn unparse<'a, 'b>(&self, stream: &mut Stream<'a, 'b>) -> Result<()> {
        let mut buf = [0u8; 4];
        let string = self.encode_utf8(&mut buf);
        string.unparse(stream)
    }
}

impl<T> Unparse for Box<T>
where
    T: Unparse,
{
    fn unparse<'a, 'b>(&self, stream: &mut Stream<'a, 'b>) -> Result<()> {
        self.deref().unparse(stream)
    }
}
