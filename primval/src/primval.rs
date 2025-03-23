use derive_more::{From, TryInto};

/// A [PrimVal] is a value the language inherently provides which excludes containing other values or value-references
///
/// Note that some [PrimVal] values _can_ be containers of [PrimVal] types. For example, a string contains chars, and a char is also a [PrimVal].
#[derive(Copy, Clone, Debug, PartialEq, From, TryInto)]
pub enum PrimVal {
    #[from(Num, i32)]
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

// TODO: This kind of stuff should migrate to a dedicated Num crate

/// A number value
///
/// # TODO
///
/// ints (including bytes), big ints, decimals...
pub type Num = f64;

impl PartialEq<f64> for PrimVal {
    fn eq(&self, other: &f64) -> bool {
        match self {
            PrimVal::Num(f) => f.eq(other),
        }
    }
}

impl PartialEq<i32> for PrimVal {
    fn eq(&self, ir: &i32) -> bool {
        self.eq(&Num::from(*ir))
    }
}

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
