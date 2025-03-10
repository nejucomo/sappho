/// A [PrimVal] is a value the language inherently provides which excludes containing other values or value-references
///
/// Note that some [PrimVal] values _can_ be containers of [PrimVal] types. For example, a string contains chars, and a char is also a [PrimVal].
pub enum PrimVal {
    Num(Num),
    // TODO:
    //Bool(bool),
    //Char(char),

    // Less certain. Here we start heap allocations. We might want a distinction?
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
