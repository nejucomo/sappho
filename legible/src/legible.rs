use crate::fmtpos::FmtPos;
use crate::ldisp::LegibleDisplay;
use crate::{IntoNode, DEFAULT_FMT_WIDTH_THRESHOLD};

/// `Legible` types are represented compactly until they pass a width threshold, in which case their display uses newlines and indentation
///
/// # Displaying Legibles
///
/// Typically `Legible` types delegate from `Display`:
///
/// ```
/// use std::fmt;
/// use legible::{Legible, IntoNode, Node};
///
/// struct MyType {
///   // ...
/// }
///
/// impl<'a> IntoNode<'a> for &'a MyType {
///     fn into_node(self) -> Node<'a> {
///         todo!("implement legible specification for MyType")
///     }
/// }
///
/// impl fmt::Display for MyType {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         self.fmt_legible(f)
///     }
/// }
/// ```
pub trait Legible
where
    for<'a> &'a Self: IntoNode<'a>,
{
    /// Format `self` to `f` via the `Legible` specification with the default width threshold
    fn fmt_legible(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_width_threshold(f, DEFAULT_FMT_WIDTH_THRESHOLD)
    }

    /// Format self to `f` via the `Legible` `Node` specification
    ///
    /// Forms will be wrapped when the `threshold` column is reached.
    fn fmt_with_width_threshold(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        threshold: usize,
    ) -> std::fmt::Result {
        self.into_node()
            .write_to_stream(&mut FmtPos::new(f, threshold))
    }
}

impl<T> Legible for T where for<'a> &'a T: IntoNode<'a> {}
