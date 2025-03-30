use sappho_effect::ProcEffect;
use sappho_parsable::Recursive;

use crate::Wise;

pub type ProcWiseParser<'a, K> = Recursive<'a, Wise<K, ProcEffect>>;
