use crate::error::{BareError, Span};
use sappho_source::Source;
use std::fmt;

#[derive(Debug)]
pub struct SourcedError<'a> {
    source: Source<'a>,
    bare: BareError,
}

impl<'a> SourcedError<'a> {
    pub fn new(source: Source<'a>, bare: BareError) -> Self {
        SourcedError { source, bare }
    }
}

impl<'a> fmt::Display for SourcedError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::error::indent::indent;

        let (lix, lspan, lstr) = select_source(self.source.text(), self.bare.span());
        write!(
            f,
            "from {}, line {}:\n{}|\n+-> Syntax error: {}\n",
            self.source
                .path()
                .map(|p| format!("{:?}", p.display()))
                .unwrap_or_else(|| "<string>".to_string()),
            lix + 1,
            indent(
                "| ",
                &format!(
                    "{}{}\n{}",
                    lstr,
                    if lstr.trim_end().len() < lstr.len() {
                        // Show trailing whitespace indicator:
                        "<- end of line"
                    } else {
                        ""
                    },
                    make_cursor(lspan),
                )
            ),
            self.bare,
        )
    }
}

fn select_source(src: &str, span: Span) -> (usize, Span, &str) {
    let mut start = span.start;
    let mut lastlix = 0;
    let mut lastline = &src[..0];

    for (lix, line) in src.lines().enumerate() {
        if start >= line.len() {
            start -= line.len();
        } else {
            let rawend = start + (span.end - span.start);
            let end = std::cmp::min(rawend, line.len());
            return (lix, start..end, line);
        }
        lastlix = lix;
        lastline = line;
    }
    (lastlix, lastline.len()..lastline.len() + 1, lastline)
}

fn make_cursor(lspan: Span) -> String {
    let mut s = String::new();
    for _ in 0..lspan.start {
        s.push(' ');
    }
    for _ in lspan {
        s.push('^');
    }
    s
}
