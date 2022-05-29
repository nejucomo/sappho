use crate::error::{BareError, Span};
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct SourcedError {
    path: Option<PathBuf>,
    source: String,
    bare: BareError,
}

impl SourcedError {
    pub fn new(path: Option<PathBuf>, source: &str, bare: BareError) -> Self {
        let source = source.to_string();
        SourcedError { path, source, bare }
    }
}

impl fmt::Display for SourcedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::error::indent::indent;

        let (lix, lspan, lstr) = select_source(&self.source, self.bare.span());
        write!(
            f,
            "Error from {}, line {}:\n{}|\n+-> Syntax error: {}\n",
            self.path
                .as_ref()
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
    assert_eq!(start, 1);
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
