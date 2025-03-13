mod ast;
mod compiler;

use ast::{BinOp, BinOpKind, Expr, Lit};
use compiler::{Compiler as _, Interpreter};

fn main() {
    // Parse tree for binary expression `(2 + 3) * (4 + 6 - 5)`
    //                                    \   /       \   /
    //                                      5     *     5
    //                                         \     /
    //                                           25
    let expr = Expr::BinOp(BinOp::new(
        Expr::BinOp(BinOp::new(
            Expr::Lit(Lit::Int { val: 2 }),
            Expr::Lit(Lit::Int { val: 3 }),
            BinOpKind::Add,
        )),
        Expr::BinOp(BinOp::new(
            Expr::BinOp(BinOp::new(
                Expr::Lit(Lit::Int { val: 4 }),
                Expr::Lit(Lit::Int { val: 6 }),
                BinOpKind::Add,
            )),
            Expr::Lit(Lit::Int { val: 5 }),
            BinOpKind::Sub,
        )),
        BinOpKind::Mul,
    ));
    let interpreter = Interpreter::new(expr);

    let res = interpreter.compile().int_val().unwrap();
    assert_eq!(res, 25);

    println!("Result is {}", res);
}
