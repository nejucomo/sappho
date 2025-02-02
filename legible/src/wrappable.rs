use crate::stream::Stream;
use crate::writestr::WriteStr;

pub(crate) trait WrappableDisplay {
    fn write_to_stream_maybe_wrapped<W>(&self, stream: &mut Stream<W>) -> Result<(), W::Error>
    where
        W: WriteStr,
    {
        let mut ts = stream.trial();
        let needs_wrap = self.write_to_stream_with_wrap(&mut ts, false).is_err();
        self.write_to_stream_with_wrap(stream, needs_wrap)
    }

    fn write_to_stream_with_wrap<W>(
        &self,
        stream: &mut Stream<W>,
        wrap: bool,
    ) -> Result<(), W::Error>
    where
        W: WriteStr;
}

impl<T> WrappableDisplay for &T
where
    T: ?Sized + WrappableDisplay,
{
    fn write_to_stream_with_wrap<W>(
        &self,
        stream: &mut Stream<W>,
        wrap: bool,
    ) -> Result<(), W::Error>
    where
        W: WriteStr,
    {
        (*self).write_to_stream_with_wrap(stream, wrap)
    }
}
