use derive_new::new;

use crate::{Result, WrapError};

#[derive(Copy, Clone, Debug, new)]
pub struct Position {
    maxwidth: usize,
    #[new(default)]
    col: usize,
    #[new(default)]
    indent_level: usize,
    #[new(value = "2")]
    indentation_size: usize,
    #[new(default)]
    wraperr: bool,
}

impl Position {
    pub(crate) fn indent_inc(&mut self, indent: bool) {
        if indent {
            self.indent_level += 1;
        } else {
            assert!(self.indent_level > 0);
            self.indent_level -= 1;
        };
    }

    pub(crate) fn indent_level(&self) -> usize {
        self.indent_level
    }

    pub(crate) fn indentation_size(&self) -> usize {
        self.indentation_size
    }

    pub(crate) fn track_str(&mut self, s: &str) -> Result<()> {
        for c in s.chars() {
            if c == '\t' {
                panic!("tabs are evil");
            } else if c == '\n' {
                self.col = 0;
                if self.wraperr {
                    return Err(WrapError::Newline.into());
                }
            } else {
                self.col += 1;
                if self.wraperr && self.col >= self.maxwidth {
                    return Err(WrapError::TooWide {
                        column: self.col,
                        limit: self.maxwidth,
                    }
                    .into());
                }
            }
        }

        Ok(())
    }
}
