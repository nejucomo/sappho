use crate::stream::Stream;
use crate::writestr::WriteStr;

pub(crate) trait LegibleDisplay {
    fn write_to_stream<W>(&self, stream: &mut Stream<W>) -> Result<(), W::Error>
    where
        W: WriteStr;
}

impl<T> LegibleDisplay for &T
where
    T: ?Sized + LegibleDisplay,
{
    fn write_to_stream<W>(&self, stream: &mut Stream<W>) -> Result<(), W::Error>
    where
        W: WriteStr,
    {
        (*self).write_to_stream(stream)
    }
}

impl LegibleDisplay for str {
    fn write_to_stream<W>(&self, stream: &mut Stream<W>) -> Result<(), W::Error>
    where
        W: WriteStr,
    {
        stream.write_chunk(self)
    }
}

impl<T> LegibleDisplay for Box<T>
where
    T: LegibleDisplay,
{
    fn write_to_stream<W>(&self, stream: &mut Stream<W>) -> Result<(), W::Error>
    where
        W: WriteStr,
    {
        self.as_ref().write_to_stream(stream)
    }
}

impl LegibleDisplay for char {
    fn write_to_stream<W>(&self, stream: &mut Stream<W>) -> Result<(), W::Error>
    where
        W: WriteStr,
    {
        let mut buf = [0; 4];
        stream.write_chunk(self.encode_utf8(&mut buf))
    }
}
