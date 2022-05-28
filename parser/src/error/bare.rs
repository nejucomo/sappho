use chumsky::error::Error as ChumskyError;
use chumsky::error::Simple;
use derive_more::From;
use std::fmt;

#[derive(Debug, From)]
pub struct BareError(Simple<char>);

pub type Span = <Simple<char> as ChumskyError<char>>::Span;
pub type Label = <Simple<char> as ChumskyError<char>>::Label;

impl BareError {
    pub fn custom(span: Span, msg: String) -> Self {
        BareError(Simple::custom(span, msg))
    }
}

impl BareError {
    pub fn span(&self) -> Span {
        self.0.span()
    }
}

impl ChumskyError<char> for BareError {
    type Span = Span;
    type Label = Label;

    fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
        span: Self::Span,
        expected: Iter,
        found: Option<char>,
    ) -> Self {
        BareError(Simple::expected_input_found(span, expected, found))
    }

    fn with_label(self, label: Self::Label) -> Self {
        BareError(self.0.with_label(label))
    }

    fn merge(self, other: Self) -> Self {
        BareError(self.0.merge(other.0))
    }
}

impl fmt::Display for BareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use chumsky::error::SimpleReason::*;

        if let Some(label) = self.0.label() {
            write!(f, "while parsing {} ", label)?
        }

        match self.0.reason() {
            Unexpected => write!(
                f,
                "found {}, expected {}",
                describe_found(self.0.found().map(|&c| c)),
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

fn describe_found(oc: Option<char>) -> String {
    oc.map(|c| format!("{:?}", c))
        .unwrap_or_else(|| "nothing".to_string())
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
