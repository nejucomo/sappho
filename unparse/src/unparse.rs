use std::ops::Deref;

use either::Either::{self, Left, Right};

use crate::{Result, Stream};

pub trait Unparse {
    fn unparse<'a, 'b>(self, stream: &mut Stream<'a, 'b>) -> Result<()>;
}

impl<'s> Unparse for &'s str {
    fn unparse<'a, 'b>(self, stream: &mut Stream<'a, 'b>) -> Result<()> {
        stream.write_str(self)
    }
}

impl Unparse for char {
    fn unparse<'a, 'b>(self, stream: &mut Stream<'a, 'b>) -> Result<()> {
        let mut buf = [0u8; 4];
        let string = self.encode_utf8(&mut buf);
        string.unparse(stream)
    }
}

impl<'s, T> Unparse for &'s Box<T>
where
    &'s T: Unparse,
{
    fn unparse<'a, 'b>(self, stream: &mut Stream<'a, 'b>) -> Result<()> {
        self.deref().unparse(stream)
    }
}

impl<L, R> Unparse for Either<L, R>
where
    L: Unparse,
    R: Unparse,
{
    fn unparse<'a, 'b>(self, stream: &mut Stream<'a, 'b>) -> Result<()> {
        match self {
            Left(l) => l.unparse(stream),
            Right(r) => r.unparse(stream),
        }
    }
}
