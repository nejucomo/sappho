use sappho_identifier::IdentRef;

pub trait Key {
    fn as_ident_ref(&self) -> &IdentRef;
}

impl Key for IdentRef {
    fn as_ident_ref(&self) -> &IdentRef {
        self
    }
}

impl Key for &'static str {
    fn as_ident_ref(&self) -> &IdentRef {
        IdentRef::from_static(self)
    }
}
