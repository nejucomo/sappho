use crate::position::Position;
use crate::{Error, Stream, Unparse};

const DISPLAY_MAX_WIDTH: usize = 80;

pub fn to_formatter<U>(unparser: U, f: &mut std::fmt::Formatter) -> std::fmt::Result
where
    U: Unparse,
{
    let mut stream = Stream::new(Some(f), Position::new(DISPLAY_MAX_WIDTH));
    unparser.unparse(&mut stream).map_err(|e| match e {
        Error::Wrap(e) => panic!("internal inconsistency: unexpected {e:?}"),
        Error::Fmt(e) => e,
    })
}
