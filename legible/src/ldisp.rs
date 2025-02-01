use crate::stream::Stream;

pub(crate) trait LegibleDisplay {
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream;
}

impl<'a, T> LegibleDisplay for &'a T
where
    T: ?Sized + LegibleDisplay,
{
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        (*self).write_to_stream(stream)
    }
}

impl LegibleDisplay for str {
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        stream.write_chunk(self)
    }
}

impl<T> LegibleDisplay for Box<T>
where
    T: LegibleDisplay,
{
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        self.as_ref().write_to_stream(stream)
    }
}

impl LegibleDisplay for char {
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        let mut buf = [0; 4];
        stream.write_chunk(self.encode_utf8(&mut buf))
    }
}
