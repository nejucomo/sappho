#[derive(Copy, Clone, Debug)]
pub(crate) struct Indentation {
    level: usize,
    width: usize,
}

impl Default for Indentation {
    fn default() -> Self {
        Indentation { level: 0, width: 2 }
    }
}

impl Indentation {
    pub(crate) fn indent(&mut self) {
        self.level += 1;
    }

    pub(crate) fn dedent(&mut self) {
        assert!(self.level > 0);
        self.level -= 1;
    }

    pub(crate) fn column(&self) -> usize {
        self.level * self.width
    }
}
