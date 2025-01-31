use std::borrow::Cow;

use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;

/// A chunk of text that excludes newlines
#[derive(Debug)]
pub struct Text<'a>(Cow<'a, str>);

/// Indicates why a bare `&str` or `String` is rejected: '\n' and '\t' are forbidden
#[derive(Debug, thiserror::Error)]
#[error("Text values may not include {invalid_char:?} characters: {raw_text:?}")]
pub struct TextError<'a> {
    invalid_char: char,
    raw_text: Cow<'a, str>,
}

impl<'a> Text<'a> {
    /// An `&str` of this text
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    /// The character count
    pub fn width(&self) -> usize {
        self.as_ref().chars().count()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Text<'a> {
    type Error = TextError<'a>;

    fn try_from(raw_text: Cow<'a, str>) -> Result<Self, Self::Error> {
        let s = raw_text.as_ref();
        if let Some(ix) = s.find(['\n', '\t']) {
            let invalid_char = s[ix..].chars().next().unwrap();
            Err(TextError {
                raw_text,
                invalid_char,
            })
        } else {
            Ok(Text(raw_text))
        }
    }
}

impl<'a> TryFrom<&'a str> for Text<'a> {
    type Error = TextError<'a>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(Cow::from(s))
    }
}

impl<'a> TryFrom<String> for Text<'a> {
    type Error = TextError<'a>;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(Cow::from(s))
    }
}

impl<'a> AsRef<str> for Text<'a> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> LegibleDisplay for Text<'a> {
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        stream.write(self.as_str())
    }
}
