use sappho_listform::ListForm;
use sappho_parsable::{ParsableWith as _, Parser};
use sappho_unparse::Unparse;

/// Temporary shim for [sappho_parsable] transition
pub(crate) fn list_form<PX, PT, X, T>(item: PX, tail: PT) -> impl Parser<ListForm<X, T>>
where
    X: Unparse + std::fmt::Debug,
    T: Unparse + std::fmt::Debug,
    PX: Parser<X>,
    PT: Parser<T> + Clone,
{
    ListForm::parser_with((item, tail))
}
