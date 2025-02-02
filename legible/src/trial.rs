use crate::position::OverflowError;
use crate::writestr::WriteStr;

#[derive(Debug)]
pub(crate) struct Trial;

impl WriteStr for Trial {
    type Error = OverflowError;

    fn write_str_over_threshold(
        &mut self,
        _: &str,
        overflow: Option<OverflowError>,
    ) -> Result<(), Self::Error> {
        overflow.map(Err).unwrap_or(Ok(()))
    }
}
