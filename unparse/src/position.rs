use derive_new::new;

#[derive(Copy, Clone, Debug, new)]
pub struct Position {
    maxwidth: usize,
    #[new(default)]
    col: usize,
    #[new(default)]
    indent_level: usize,
    #[new(value = "2")]
    indentation_size: usize,
}

impl Position {
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn dedent(&mut self) {
        assert!(self.indent_level > 0);
        self.indent_level -= 1;
    }

    pub(crate) fn indentation_column(&self) -> usize {
        self.indent_level * self.indentation_size
    }

    pub(crate) fn track_str(&mut self, s: &str) -> bool {
        let mut wrapped = false;
        for c in s.chars() {
            if c == '\t' {
                panic!("tabs are evil");
            } else if c == '\n' {
                self.col = 0;
                wrapped = true;
            } else {
                self.col += 1;
                wrapped |= self.col >= self.maxwidth;
            }
        }

        wrapped
    }
}
