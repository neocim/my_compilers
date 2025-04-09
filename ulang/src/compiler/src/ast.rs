pub mod token;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Punct {
    Colon,     // `:`
    SemiColon, // `;`
    Comma,     // `,`
}

/// We can consider these to be something like reserved tokens.
/// We do not currently support these tokens, but they may be available in the future.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Invalid {
    SigleOr, // just `|`
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Delimiter {
    OBrace,   // `{`
    CBrace,   // `}`
    OParen,   // `(`
    CParen,   // `)`
    OBracket, // `[`
    CBracket, // `]`
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AssignOp {
    Assign,    // `=`
    MulAssign, // `*=`
    ModAssign, // `%=`
    DivAssign, // `/=`
    SubAssign, // `-=`
    AddAssign, // `+=`
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinOp {
    Add,         // `+`
    Sub,         // `-`
    Mul,         // `*`
    Div,         // `/`
    Mod,         // `%`
    LessThan,    // `<`
    LtOrEq,      // `<=`
    GreaterThan, // `>`
    GtOrEq,      // `>=`
    NotEq,       // `!=`
    Eq,          // `==`
    Or,          // `||`
    And,         // `&&`
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnOp {
    Ref,   // `&`
    Deref, // `*`
    Not,   // `!`
    UnAdd, // `++`
    UnSub, // `--`
}
