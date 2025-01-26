use crate::position::Position;
use crate::{Result, Unparse};

pub trait Stream: Sized {
    fn write<U>(&mut self, value: U) -> Result<()>
    where
        U: Unparse,
    {
        value.unparse(self)
    }

    fn position(&self) -> Position;

    fn newline(&mut self, indent: bool) -> Result<()> {
        let p = self.pos_mut();
        p.inc_indentation(indent);
        self.write_str("\n")?;
        p.write_indentation(self)
    }

    #[doc(hidden)]
    fn write_str(&mut self, s: &str) -> Result<()>;

    #[doc(hidden)]
    fn pos_mut(&mut self) -> &mut Position;
}
