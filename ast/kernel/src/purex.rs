//! TODO: Come up with a more specific name for this effect.

use crate::{Kernel, Root};

/// A [Pure] is an expression without effects
#[derive(Debug, derive_more::From)]
pub struct PureX(Kernel<PureX>);

impl Root for PureX {}
