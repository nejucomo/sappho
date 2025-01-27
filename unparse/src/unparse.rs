use either::Either::{self, Left, Right};

use crate::{Result, Stream};

pub trait Unparse {
    /// Write the unparse of `self` into `stream` and return if we wrapped
    fn unparse<'a, 'b>(self, stream: &mut Stream<'a, 'b>) -> Result<bool>;
}

impl<'s> Unparse for &'s str {
    fn unparse<'a, 'b>(self, stream: &mut Stream<'a, 'b>) -> Result<bool> {
        stream.write_str(self)
    }
}

impl Unparse for char {
    fn unparse<'a, 'b>(self, stream: &mut Stream<'a, 'b>) -> Result<bool> {
        let mut buf = [0u8; 4];
        let string = self.encode_utf8(&mut buf);
        string.unparse(stream)
    }
}

impl<L, R> Unparse for Either<L, R>
where
    L: Unparse,
    R: Unparse,
{
    fn unparse<'a, 'b>(self, stream: &mut Stream<'a, 'b>) -> Result<bool> {
        match self {
            Left(l) => l.unparse(stream),
            Right(r) => r.unparse(stream),
        }
    }
}

impl<A, B> Unparse for (A, B)
where
    A: Unparse,
    B: Unparse,
{
    fn unparse<'a, 'b>(self, stream: &mut Stream<'a, 'b>) -> Result<bool> {
        let (a, b) = self;
        let w = stream.write(a)?;
        let w = w | stream.write(b)?;
        Ok(w)
    }
}

impl<A, B, C> Unparse for (A, B, C)
where
    A: Unparse,
    B: Unparse,
    C: Unparse,
{
    fn unparse<'a, 'b>(self, stream: &mut Stream<'a, 'b>) -> Result<bool> {
        let (a, b, c) = self;
        (a, (b, c)).unparse(stream)
    }
}
