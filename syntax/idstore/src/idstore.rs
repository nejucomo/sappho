use std::collections::BTreeMap;

use sappho_syntax_identifier::InvalidIdentifier;

use crate::ArcId;

#[derive(Debug, Default)]
pub struct IdStore(BTreeMap<ArcId, ArcId>);

impl IdStore {
    pub fn resolve<S>(&mut self, candidate: S) -> Result<ArcId, InvalidIdentifier>
    where
        String: From<S>,
    {
        let key = ArcId::new(String::from(candidate))?;
        let rcidref = self.0.entry(key.clone()).or_insert(key);
        Ok(rcidref.clone())
    }
}
