use derive_more::From;

use crate::position::Position;
use crate::stream::Stream;

#[derive(Debug, From)]
pub(crate) struct IndentStream<'s, S>(&'s mut S);

impl<'s, S> IndentStream<'s, S> {
    pub(crate) fn dedent(self) -> &'s mut S {
        self.0
    }
}

impl<S> Stream for IndentStream<'_, S>
where
    S: Stream,
{
    type Error = S::Error;

    fn position(&self) -> Position {
        self.0.position()
    }

    fn write_chunk(&mut self, chunk: &str) -> Result<(), Self::Error> {
        self.0.write_chunk(chunk)
    }

    fn write_newline(&mut self) -> Result<(), Self::Error> {
        self.0.write_newline()?;
        self.0.write_chunk("  ")?;
        Ok(())
    }
}
