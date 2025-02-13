#![cfg(test)]

use std::collections::VecDeque;

use crate::{
    ast::{
        token::{BinOpKind, LiteralKind as AstLiteralKind, Token as AstToken},
        TokenStream,
    },
    helpers::test::DebugHelper,
    lexer::{
        cursor::Cursor,
        token::{LiteralKind, Token},
        Lexer,
    },
};

#[test]
fn cursor_test() {
    let input = "123.4 * 5 + (6789.12345 - 600)";
    let mut lexer = Cursor::new(input);
    let token_stream: Vec<_> = lexer.tokenize().collect();
    let token_stream = DebugHelper::new(&token_stream);

    assert_eq!(
        token_stream,
        DebugHelper::new(
            vec![
                // `123.4 * 5 + `
                Token::Lit {
                    kind: LiteralKind::Float {
                        val: "123.4".to_string(),
                    },
                },
                Token::Whitespace,
                Token::Star,
                Token::Whitespace,
                Token::Lit {
                    kind: LiteralKind::Int {
                        val: "5".to_string(),
                    },
                },
                Token::Whitespace,
                Token::Plus,
                Token::Whitespace,
                // `(6789.12345 - 600)`
                Token::OpenParen,
                Token::Lit {
                    kind: LiteralKind::Float {
                        val: "6789.12345".to_string(),
                    },
                },
                Token::Whitespace,
                Token::Minus,
                Token::Whitespace,
                Token::Lit {
                    kind: LiteralKind::Int {
                        val: "600".to_string(),
                    },
                },
                Token::CloseParen,
            ]
            .as_ref()
        )
    )
}

#[test]
fn lexer_token_stream_test() {
    let mut lexer = Lexer::new("123 + 54321 - ( 1.123456789 ) * 3 / 1 % 10");
    let result = lexer.token_stream();

    assert_eq!(
        DebugHelper::new(&result.0),
        DebugHelper::new(&VecDeque::from([
            // `123 + 54321 -`
            AstToken::Lit {
                kind: AstLiteralKind::Int {
                    val: "123".to_string(),
                },
            },
            AstToken::BinOp(BinOpKind::Add),
            AstToken::Lit {
                kind: AstLiteralKind::Int {
                    val: "54321".to_string(),
                },
            },
            AstToken::BinOp(BinOpKind::Sub),
            // `( 1.123456789 ) * 3 /`
            AstToken::OpenParen,
            AstToken::Lit {
                kind: AstLiteralKind::Float {
                    val: "1.123456789".to_string(),
                },
            },
            AstToken::CloseParen,
            AstToken::BinOp(BinOpKind::Mul),
            AstToken::Lit {
                kind: AstLiteralKind::Int {
                    val: "3".to_string(),
                },
            },
            AstToken::BinOp(BinOpKind::Div),
            // `1 % 10`
            AstToken::Lit {
                kind: AstLiteralKind::Int {
                    val: "1".to_string(),
                },
            },
            AstToken::BinOp(BinOpKind::Mod),
            AstToken::Lit {
                kind: AstLiteralKind::Int {
                    val: "10".to_string(),
                },
            },
        ]))
    );
}

#[test]
fn test_token_stream_next() {
    let mut stream = TokenStream::new(VecDeque::from([
        AstToken::BinOp(BinOpKind::Add),
        AstToken::BinOp(BinOpKind::Sub),
        AstToken::BinOp(BinOpKind::Div),
    ]));

    assert_eq!(stream.next(), Some(AstToken::BinOp(BinOpKind::Add)));
    assert_eq!(stream.next(), Some(AstToken::BinOp(BinOpKind::Sub)));
    assert_eq!(stream.next(), Some(AstToken::BinOp(BinOpKind::Div)));
    // make sure that we are not advancing in the cloned field
    assert_eq!(stream.0.into_iter().next(), None);
}
