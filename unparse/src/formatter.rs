use crate::position::Position;
use crate::{Error, Stream, Unparse};

pub fn to_formatter<U>(
    unparser: &U,
    f: &mut std::fmt::Formatter,
    max_width: usize,
) -> std::fmt::Result
where
    U: Unparse,
{
    let mut stream = Stream::new(Some(f), Position::new(max_width));
    unparser.unparse(&mut stream).map_err(|e| match e {
        Error::Wrap(e) => panic!("internal inconsistency: unexpected {e:?}"),
        Error::Fmt(e) => e,
    })
}
