use derive_new::new;
use sappho_ast_effect::ProcEffect;
use sappho_parsable::Recursive;
use sappho_source::SourceCodeLink;

use crate::Wise;

#[derive(Clone, new)]
pub(crate) struct ParseParams<'a> {
    pub(crate) recp: Recursive<'a, Wise<ProcEffect>>,
    pub(crate) sclink: SourceCodeLink,
}
