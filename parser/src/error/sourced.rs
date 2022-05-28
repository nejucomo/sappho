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
        self.bare.fmt(f)?;

        let (lix, lspan, lstr) = select_source(&self.source, self.bare.span());
        write!(
            f,
            "\nIn source {}, line {}:\n  {}\n  {}",
            self.path
                .as_ref()
                .map(|p| format!("{:?}", p.display()))
                .unwrap_or_else(|| "<string>".to_string()),
            lix + 1,
            lstr,
            make_cursor(lspan),
        )
    }
}

fn select_source(src: &str, span: Span) -> (usize, Span, &str) {
    let mut start = span.start;
    for (lix, line) in src.lines().enumerate() {
        if start > line.len() {
            start -= line.len();
        } else {
            let rawend = start + (span.end - span.start);
            let end = std::cmp::min(rawend, line.len());
            return (lix, start..end, line);
        }
    }
    panic!("internal span tracking failure");
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
