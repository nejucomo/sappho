use std::fmt::Formatter;

use crate::{Error, Stream, Unparse};

const DISPLAY_MAX_WIDTH: usize = 80;

pub fn to_formatter<U>(unparser: U, f: &mut Formatter) -> std::fmt::Result
where
    U: Unparse,
{
    to_formatter_with_max_width(unparser, f, DISPLAY_MAX_WIDTH)
}

pub fn to_formatter_with_max_width<U>(
    unparser: U,
    f: &mut Formatter,
    max_width: usize,
) -> std::fmt::Result
where
    U: Unparse,
{
    let mut stream = Stream::new(f, max_width);
    unparser
        .unparse(&mut stream)
        .map(|_| ())
        .map_err(|e| match e {
            Error::Wrapped => panic!("internal inconsistency: unexpected {e:?}"),
            Error::Fmt(e) => e,
        })
}
