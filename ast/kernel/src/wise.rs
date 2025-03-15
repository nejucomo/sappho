use derive_more::From;
use sappho_ast_effect::Effect;
use sappho_with_source::WithSource;

use crate::AstProvider;

/// WIth Source Expression is the top-level recursion entrypoint for AST expressions
#[derive(Debug, From)]
pub struct Wise<XP, FX>(WithSource<XP::Expr<FX>>)
where
    XP: AstProvider,
    FX: Effect;

/// Heap-allocated [Wise]
#[allow(dead_code)]
#[derive(Debug)]
pub struct BoxWise<XP, FX>(Box<Wise<XP, FX>>)
where
    XP: AstProvider,
    FX: Effect;
