#![cfg(test)]

use crate::{token::Token, Lexer};

#[test]
fn lexer_tests() {
    let input = "123.4 * 5 + (6789.10 - 11.12131415)";
    let mut lexer = Lexer::new(input);
    let token_stream: Vec<Token> = lexer.tokenize().collect();

    assert_eq!(token_stream, Vec::new())
}
