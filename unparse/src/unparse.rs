use std::ops::Deref;

use crate::{Result, Stream};

pub trait Unparse {
    fn unparse<S>(&self, stream: &mut S) -> Result<()>
    where
        S: Stream;
}

impl Unparse for str {
    fn unparse<S>(&self, stream: &mut S) -> Result<()>
    where
        S: Stream,
    {
        stream.write_str(self)
    }
}

impl Unparse for char {
    fn unparse<S>(&self, stream: &mut S) -> Result<()>
    where
        S: Stream,
    {
        let mut buf = [0u8; 4];
        let string = self.encode_utf8(&mut buf);
        string.unparse(stream)
    }
}

impl<T> Unparse for Box<T>
where
    T: Unparse,
{
    fn unparse<S>(&self, stream: &mut S) -> Result<()>
    where
        S: Stream,
    {
        self.deref().unparse(stream)
    }
}
