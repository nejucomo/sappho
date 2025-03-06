use std::sync::{LazyLock, Mutex};

use sappho_syntax_identifier::InvalidIdentifier;

use crate::idstore::IdStore;
use crate::ArcId;

pub fn resolve<S>(candidate: S) -> Result<ArcId, InvalidIdentifier>
where
    String: From<S>,
{
    static STORE: LazyLock<Mutex<IdStore>> = LazyLock::new(|| Mutex::new(IdStore::default()));

    let mut store = STORE.lock().unwrap();
    store.resolve(candidate)
}

/// The caller guarantees `ident` is a valid identifier
pub fn resolve_static(ident: &'static str) -> ArcId {
    resolve(ident).unwrap()
}
