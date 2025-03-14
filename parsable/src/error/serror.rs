use std::fmt;

use chumsky::error::Simple;
use sappho_source::Span;

#[derive(Debug, derive_more::From)]
pub struct ChumskyError(Simple<char, Span>);

impl ChumskyError {
    pub fn custom<M: ToString>(span: Span, msg: M) -> Self {
        ChumskyError(Simple::custom(span, msg))
    }

    pub fn span(&self) -> Span {
        self.0.span()
    }
}

impl chumsky::Error<char> for ChumskyError {
    type Span = Span;
    type Label = &'static str;

    fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
        span: Self::Span,
        expected: Iter,
        found: Option<char>,
    ) -> Self {
        ChumskyError(Simple::expected_input_found(span, expected, found))
    }

    fn with_label(self, label: Self::Label) -> Self {
        ChumskyError(self.0.with_label(label))
    }

    fn merge(self, other: Self) -> Self {
        ChumskyError(self.0.merge(other.0))
    }
}

impl fmt::Display for ChumskyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use chumsky::error::SimpleReason::*;

        if let Some(label) = self.0.label() {
            write!(f, "while parsing {}, ", label)?;
        }

        match self.0.reason() {
            Unexpected => write!(
                f,
                "unexpected {} while expecting {}",
                self.0
                    .found()
                    .map(debug)
                    .unwrap_or_else(|| "end of input".to_string()),
                comma_separated_or(self.0.expected().filter_map(|&x| x).map(debug)),
            ),
            Unclosed { span, delimiter } => write!(f, "unclosed {:?} at {:?}", delimiter, span),
            Custom(msg) => write!(f, "{}", msg),
        }
    }
}

fn debug<T>(x: T) -> String
where
    T: std::fmt::Debug,
{
    format!("{:?}", x)
}

fn comma_separated_or<I>(it: I) -> String
where
    I: Iterator<Item = String>,
{
    let mut items: Vec<String> = it.collect();
    items.sort();

    match items.len() {
        0 => panic!("expected empty set!"),
        1 => items.into_iter().next().unwrap(),
        2 => format!("{} or {}", &items[0], &items[1]),
        p => format!("{}, or {}", items[0..p - 1].join(", "), &items[p - 1]),
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(&["a"] => "a".to_string())]
    #[test_case(&["a", "b"] => "a or b".to_string())]
    #[test_case(&["a", "b", "c"] => "a, b, or c".to_string())]
    fn comma_separated_or(xs: &[&str]) -> String {
        super::comma_separated_or(xs.iter().map(|x| x.to_string()))
    }
}
