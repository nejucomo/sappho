use sappho_with_source::WithSource;

use crate::Expr;

/// **Wi**th **S**ource **E**xpression
///
/// An [Expr] with attached source code for errors and other diagnostics.
///
/// This is the top-level expression recursion point, without `Box`. Expression recursions to [Wise] are either through [Box] or another container, like [ListForm](sappho_listform::ListForm).
#[derive(Debug)]
pub struct Wise<FX>(WithSource<Expr<FX>>);
