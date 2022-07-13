use super::Item::{self, *};
use crate::Stream;
use std::fmt;

impl fmt::Display for Stream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_items(self, f, 0)
    }
}

fn fmt_items(stream: &Stream, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
    for item in &stream.0[..] {
        fmt_item(item, f, depth)?;
    }
    Ok(())
}

fn fmt_item(item: &Item, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
    match item {
        Leaf(s) => f.write_str(&s),
        Break(_) => {
            f.write_str("\n")?;
            for _ in 0..depth {
                f.write_str("  ")?;
            }
            Ok(())
        }
        Substream(sub) => fmt_items(sub, f, depth + 1),
    }
}
