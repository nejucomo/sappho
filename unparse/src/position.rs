use derive_new::new;

use crate::{Result, Stream, WrapError};

#[derive(Copy, Clone, Debug, new)]
pub struct Position {
    maxwidth: usize,
    #[new(default)]
    indent: usize,
    #[new(default)]
    col: usize,
    #[new(default)]
    wraperr: bool,
}

impl Position {
    pub fn wrap_trial(&self) -> Self {
        Position {
            wraperr: true,
            ..*self
        }
    }
}

impl Stream for Position {
    fn write_str(&mut self, s: &str) -> Result<()> {
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

    fn position(&self) -> Position {
        *self
    }
}
