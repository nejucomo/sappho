use crate::indentation::Indentation;

#[derive(Copy, Clone, Debug, derive_new::new)]
pub(crate) struct Position {
    max_width: usize,
    #[new(default)]
    col: usize,
    #[new(default)]
    pub(crate) indentation: Indentation,
}

impl Position {
    pub(crate) fn track_chunk(&mut self, chunk: &str) {
        self.col += chunk.chars().count();
    }

    pub(crate) fn set_column_to_indentation(&mut self) {
        self.col = self.indentation.column();
    }

    pub(crate) fn column(&self) -> usize {
        self.col
    }

    pub(crate) fn past_max_width(&self) -> bool {
        self.col >= self.max_width
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new(80)
    }
}
