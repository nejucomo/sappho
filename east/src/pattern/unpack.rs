use crate::Pattern;
use sappho_ast as ast;
use sappho_identmap::{IdentMap, IdentRef, Identifier};
use std::fmt;
use std::ops::Deref;

#[derive(Clone, Debug, Default, PartialEq, derive_more::From)]
pub struct UnpackPattern(IdentMap<Pattern>);

impl UnpackPattern {
    pub fn as_list_pattern(&self) -> Option<(Vec<&Pattern>, Option<&IdentRef>)> {
        self.try_as_list_pattern().ok()
    }

    fn try_as_list_pattern(&self) -> Result<(Vec<&Pattern>, Option<&IdentRef>), ()> {
        fn get<'a>(up: &'a UnpackPattern, attr: &IdentRef) -> Result<&'a Pattern, ()> {
            up.get(attr).ok_or(())
        }

        let mut pats = vec![];
        let mut up = self;
        loop {
            if up.is_empty() {
                return Ok((pats, None));
            } else if up.len() != 2 {
                return Err(());
            }

            pats.push(get(up, "head")?);
            match get(up, "tail")? {
                Pattern::Bind(b) => {
                    return Ok((pats, Some(b)));
                }
                Pattern::Unpack(nextup) => {
                    up = nextup;
                }
                _ => {
                    return Err(());
                }
            }
        }
    }
}

impl From<ast::UnpackPattern> for UnpackPattern {
    fn from(aup: ast::UnpackPattern) -> Self {
        UnpackPattern::from(aup.unwrap().into_map_values(Pattern::from))
    }
}

impl From<UnpackPattern> for ast::UnpackPattern {
    fn from(eup: UnpackPattern) -> Self {
        ast::UnpackPattern::from(eup.0.into_map_values(ast::Pattern::from))
    }
}

impl FromIterator<(Identifier, Pattern)> for UnpackPattern
where
    IdentMap<Pattern>: FromIterator<(Identifier, Pattern)>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Identifier, Pattern)>,
    {
        UnpackPattern(IdentMap::from_iter(iter))
    }
}

impl Deref for UnpackPattern {
    type Target = IdentMap<Pattern>;

    fn deref(&self) -> &IdentMap<Pattern> {
        &self.0
    }
}

impl fmt::Display for UnpackPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
