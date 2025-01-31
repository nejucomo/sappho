#[derive(Copy, Clone, Debug)]
pub(crate) struct Indentation {
    pub(crate) level: usize,
    pub(crate) size: usize,
}

#[derive(Debug)]
pub(crate) enum IndentationDelta {
    Indent,
    Dedent,
}

impl Indentation {
    pub(crate) fn column(&self) -> usize {
        self.level * self.size
    }

    pub(crate) fn apply_delta(&mut self, delta: IndentationDelta) {
        use IndentationDelta::*;

        self.level = match delta {
            Indent => self.level + 1,
            Dedent => {
                assert!(self.level > 0);
                self.level - 1
            }
        }
    }
}

impl Default for Indentation {
    fn default() -> Self {
        Indentation { level: 0, size: 2 }
    }
}
