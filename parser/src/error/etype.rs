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

        let labeldesc = if let Some(label) = self.0.label() {
            format!(", while parsing {}", label)
        } else {
            "".to_string()
        };

        write!(f, "at {:?}{}: ", self.0.span(), labeldesc)?;

        match self.0.reason() {
            Unexpected => write!(
                f,
                "found {}, expected {}",
                describe_found(self.0.found().map(|&c| c)),
                comma_separated_or(self.0.expected().map(|&oc| describe_found(oc))),
            ),
            Unclosed { span, delimiter } => write!(f, "unclosed {:?} at {:?}", delimiter, span),
            Custom(msg) => write!(f, "{}", msg),
        }
    }
}

fn describe_found(oc: Option<char>) -> String {
    oc.map(|c| format!("{:?}", c))
        .unwrap_or_else(|| "nothing".to_string())
}

fn comma_separated_or<I>(mut it: I) -> String
where
    I: Iterator<Item = String>,
{
    let mut s = String::new();
    if let Some(mut prev) = it.next() {
        let mut ix = 0;
        while let Some(x) = it.next() {
            if ix > 0 {
                s.push_str(", ");
            }
            s.push_str(&prev);
            prev = x;
            ix += 1;
        }
        if !s.is_empty() {
            s.push_str(", or ");
        }
        s.push_str(&prev);
    }
    s
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(&[] => "".to_string())]
    #[test_case(&["a"] => "a".to_string())]
    #[test_case(&["a", "b"] => "a, or b".to_string())]
    #[test_case(&["a", "b", "c"] => "a, b, or c".to_string())]
    fn comma_separated_or(xs: &[&str]) -> String {
        super::comma_separated_or(xs.iter().map(|x| x.to_string()))
    }
}
