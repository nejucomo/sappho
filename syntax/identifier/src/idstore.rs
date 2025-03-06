use std::collections::BTreeMap;
use std::sync::{LazyLock, Mutex};

use crate::{ArcId, InvalidIdentifier};

pub fn resolve<S>(candidate: S) -> Result<ArcId, InvalidIdentifier>
where
    String: From<S>,
{
    static STORE: LazyLock<Mutex<IdStore>> = LazyLock::new(|| Mutex::new(IdStore::default()));

    let mut store = STORE.lock().unwrap();
    store.resolve(candidate)
}

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
