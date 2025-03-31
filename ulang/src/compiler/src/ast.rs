pub mod token;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Delimiter {
    Brace,   // `{` or `}`
    Paren,   // `(` or `)`
    Bracket, // `[` or `]`
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
