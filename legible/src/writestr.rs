use crate::position::{OverflowError, Position};

pub(crate) trait WriteStr {
    type Error;

    fn write_pos_str(&mut self, pos: &mut Position, s: &str) -> Result<(), Self::Error> {
        let overflow = pos.track(s).err();
        self.write_str_over_threshold(s, overflow)
    }

    fn write_str_over_threshold(
        &mut self,
        s: &str,
        overflow: Option<OverflowError>,
    ) -> Result<(), Self::Error>;
}

impl WriteStr for &mut std::fmt::Formatter<'_> {
    type Error = std::fmt::Error;

    fn write_str_over_threshold(
        &mut self,
        s: &str,
        _: Option<OverflowError>,
    ) -> Result<(), Self::Error> {
        self.write_str(s)
    }
}
