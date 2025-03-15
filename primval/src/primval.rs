use derive_enum_from_into::{EnumFrom, EnumTryInto};

/// A [PrimVal] is a value the language inherently provides which excludes containing other values or value-references
///
/// Note that some [PrimVal] values _can_ be containers of [PrimVal] types. For example, a string contains chars, and a char is also a [PrimVal].
#[derive(Copy, Clone, Debug, PartialEq, EnumFrom, EnumTryInto)]
pub enum PrimVal {
    Num(Num),
    // TODO:
    //Bool(bool),
    //Char(char),

    // Less certain. Here we start heap allocations and lose [Copy]. We might want a distinction?
    // However, we do not need GC-coverage beyond rust [Box]/[Rc]/[Arc].
    //String(String),

    // Even less certain. Here is a partial-heterogenous container but it still excludes non-heap GC needs.
    //PrimArray(Vec<PrimVal>) ?
    // Or alternatively, homogenous containers?
    //ByteArray(Vec<u8>)
    //U32Array(Vec<u32>)
    // ... etc?
}

/// A number value
///
/// # TODO
///
/// ints (including bytes), big ints, decimals...
pub type Num = f64;

mod parsing {
    use chumsky::Parser as _;
    use sappho_parsable::{Parsable, Parser};
    use sappho_unparse::Unparse;

    use crate::PrimVal::{self, *};

    impl Parsable for PrimVal {
        fn parser() -> impl Parser<Self> {
            crate::parseutil::number().map(Num)
        }
    }

    impl Unparse for PrimVal {
        fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
            match self {
                Num(n) => s.write(&n.to_string()),
            }
        }
    }
}
