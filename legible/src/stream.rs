use derive_new::new;

use crate::ldisp::LegibleDisplay;
use crate::position::Position;
use crate::trial::Trial;
use crate::wrappable::WrappableDisplay;
use crate::writestr::WriteStr;

#[derive(Clone, Debug, new)]
pub(crate) struct Stream<W> {
    ws: W,
    #[new(default)]
    pos: Position,
}

impl<W> Stream<W>
where
    W: WriteStr,
{
    pub(crate) fn new_with_threshold(ws: W, threshold: usize) -> Self {
        Stream {
            ws,
            pos: Position::new(threshold),
        }
    }

    pub(crate) fn write<L>(&mut self, value: L) -> Result<(), W::Error>
    where
        L: LegibleDisplay,
    {
        value.write_to_stream(self)
    }

    pub(crate) fn write_wrap<L>(&mut self, value: L, wrap: bool) -> Result<(), W::Error>
    where
        L: WrappableDisplay,
    {
        value.write_to_stream_with_wrap(self, wrap)
    }

    pub(crate) fn indent(&mut self) {
        self.pos.indentation.indent();
    }

    pub(crate) fn dedent(&mut self) {
        self.pos.indentation.dedent();
    }

    pub(crate) fn write_chunk(&mut self, chunk: &str) -> Result<(), W::Error> {
        self.write_raw(chunk)
    }

    pub(crate) fn write_joint(&mut self, j: &str, wrap: bool) -> Result<(), W::Error> {
        if wrap {
            self.write_newline()
        } else {
            self.write_chunk(j)
        }
    }

    pub(crate) fn write_newline(&mut self) -> Result<(), W::Error> {
        self.write_raw("\n")?;
        for _ in 0..self.pos.indentation.column() {
            self.write_raw(" ")?;
        }
        Ok(())
    }

    pub(crate) fn trial(&self) -> Stream<Trial> {
        Stream {
            ws: Trial,
            pos: self.pos,
        }
    }

    fn write_raw(&mut self, s: &str) -> Result<(), W::Error> {
        self.ws.write_pos_str(&mut self.pos, s)
    }
}
