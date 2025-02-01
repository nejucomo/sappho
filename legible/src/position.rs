#[derive(Copy, Clone, Debug, derive_new::new)]
pub(crate) struct Position {
    threshold: usize,
    #[new(default)]
    col: usize,
}

impl Position {
    pub(crate) fn reset_column(&mut self) {
        self.col = 0;
    }

    pub(crate) fn track_chunk(&mut self, chunk: &str) {
        self.col += chunk.chars().count();
    }

    pub(crate) fn column(&self) -> usize {
        self.col
    }

    pub(crate) fn past_threshold(&self) -> bool {
        self.col >= self.threshold
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new(80)
    }
}
