use crate::position::Position;
use crate::stream::Stream;

#[derive(Debug, derive_new::new)]
pub(crate) struct TrialStream(Position);

#[derive(Debug, thiserror::Error)]
pub(crate) enum OverflowError {
    #[error("overflow due to a new line")]
    Newline,
    #[error("overflow due to passing column {0}")]
    TooWide(usize),
}

impl Stream for TrialStream {
    type Error = OverflowError;

    fn position(&self) -> Position {
        self.0
    }

    fn write_chunk(&mut self, chunk: &str) -> Result<(), Self::Error> {
        self.0.track_chunk(chunk);
        if self.0.past_threshold() {
            Err(OverflowError::TooWide(self.0.column()))
        } else {
            Ok(())
        }
    }

    fn write_newline(&mut self) -> Result<(), Self::Error> {
        Err(OverflowError::Newline)
    }
}
