use std::fmt;

use sappho_source::SourceCode;

use crate::error::{ChumskyError, Span};

#[derive(Debug, derive_more::Constructor, thiserror::Error)]
pub struct ParseError {
    scode: SourceCode<String>,
    errors: Vec<ChumskyError>,
}

impl ParseError {
    fn fmt_cherr(&self, f: &mut fmt::Formatter, cherr: &ChumskyError) -> fmt::Result {
        let (lix, lspan, lstr) = select_source(self.scode.code(), cherr.span());
        write!(
            f,
            "from {}, line {}:\n{}|\n+-> Syntax error: {}\n",
            self.scode.source(),
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
            cherr,
        )
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for e in &self.errors {
            self.fmt_cherr(f, e)?;
        }
        Ok(())
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

fn indent(prefix: &str, unindented: &str) -> String {
    let mut indented = String::new();
    for line in unindented.lines() {
        indented += prefix;
        indented += line;
        indented += "\n";
    }
    indented
}
