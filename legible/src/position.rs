use crate::indentation::Indentation;

#[derive(Copy, Clone, Debug, derive_new::new)]
pub(crate) struct Position {
    threshold: usize,
    #[new(default)]
    col: usize,
    #[new(default)]
    pub(crate) indentation: Indentation,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum OverflowError {
    #[error("overflow due to a new line")]
    Newline,
    #[error("overflow due to passing column {0}")]
    TooWide(usize),
}

impl Position {
    /// Track a string and return if over threshold or newline
    pub(crate) fn track(&mut self, s: &str) -> Result<(), OverflowError> {
        use OverflowError::*;

        let (newline, s) = s
            .rsplit_once('\n')
            .map(|(_, suffix)| (true, suffix))
            .unwrap_or((false, s));

        let startcol = if newline { 0 } else { self.col };

        self.col = startcol + s.chars().count();

        if newline {
            Err(Newline)
        } else if self.col >= self.threshold {
            Err(TooWide(self.threshold))
        } else {
            Ok(())
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new(80)
    }
}
