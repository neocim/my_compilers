use crate::lexer::token::OpKind;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ASTNode {
    BinOp { kind: OpKind },
}
