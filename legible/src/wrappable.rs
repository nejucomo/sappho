use crate::stream::Stream;
use crate::trial::TrialStream;

pub(crate) trait WrappableDisplay {
    fn write_to_stream_maybe_wrapped<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        let mut ts = TrialStream::new(stream.position());
        let needs_wrap = self.write_to_stream_with_wrap(&mut ts, false).is_err();
        self.write_to_stream_with_wrap(stream, needs_wrap)
    }

    fn write_to_stream_with_wrap<S>(&self, stream: &mut S, wrap: bool) -> Result<(), S::Error>
    where
        S: Stream;
}

impl<T> WrappableDisplay for &T
where
    T: ?Sized + WrappableDisplay,
{
    fn write_to_stream_with_wrap<S>(&self, stream: &mut S, wrap: bool) -> Result<(), S::Error>
    where
        S: Stream,
    {
        (*self).write_to_stream_with_wrap(stream, wrap)
    }
}
