//! Inputs to parsing, transformation, evaluation, or other functionality that has previously caused test failures

pub const UNPACK_MISSING_ATTRS: &str = "let { a: x, b: y, c: z } = { a: 2 };\nz";
