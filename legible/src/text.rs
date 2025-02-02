use derive_more::From;

use crate::innernode::InnerNode;
use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::writestr::WriteStr;
use crate::{IntoNode, Node};

/// A chunk of text that excludes '\n' and '\t'
#[derive(Clone, Debug)]
pub struct Text(InnerText);

impl IntoNode for Text {
    fn into_node(self) -> Node {
        InnerNode::Text(self).into_node()
    }
}

impl LegibleDisplay for Text {
    fn write_to_stream<W>(&self, stream: &mut Stream<W>) -> Result<(), W::Error>
    where
        W: WriteStr,
    {
        self.0.with_str(|s| stream.write_chunk(s))
    }
}

impl<T> From<T> for Text
where
    InnerText: From<T>,
{
    fn from(inner: T) -> Self {
        let inner = InnerText::from(inner);
        inner.validate_or_panic();
        Text(inner)
    }
}

#[derive(Clone, Debug, From)]
enum InnerText {
    Char(char),
    String(String),
}

impl<'a> From<&'a str> for InnerText {
    fn from(s: &'a str) -> Self {
        Self::from(s.to_string())
    }
}

impl<'a> From<&'a String> for InnerText {
    fn from(s: &'a String) -> Self {
        Self::from(s.clone())
    }
}

impl InnerText {
    pub(crate) fn with_str<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R,
    {
        use InnerText::*;

        match self {
            Char(c) => {
                let mut buf = [0; 4];
                f(c.encode_utf8(&mut buf))
            }
            String(s) => f(s.as_str()),
        }
    }

    fn validate_or_panic(&self) {
        self.with_str(|s| {
            if let Some(i) = s.find(['\n', '\t']) {
                let c = s[i..].chars().next().unwrap();
                panic!("{self:?} contains invalid char {c:?} at index {i}");
            }
        });
    }
}
