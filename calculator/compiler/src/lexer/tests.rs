#![cfg(test)]

use crate::{
    helpers::test::DebugHelper,
    lexer::{
        token::{LiteralKind, Token, TokenKind},
        cursor::Cursor,
    },
};

#[test]
fn cursor_test() {
    let input = "123.4 * 5 + (6789.12345 - 600)";
    let mut lexer = Cursor::new(input);
    let token_stream: Vec<_> = lexer.tokenize().collect();
    let token_stream = DebugHelper::new(&token_stream);

    // Comments for easier debugging
    assert_eq!(
        token_stream,
        DebugHelper::new(
            vec![
                // `123.4 * 5 + `
                Token::new(TokenKind::Lit {
                    kind: LiteralKind::Float {
                        val: "123.4".to_string(),
                    },
                }),
                Token::new(TokenKind::Whitespace),
                Token::new(TokenKind::Star),
                Token::new(TokenKind::Whitespace),
                Token::new(TokenKind::Lit {
                    kind: LiteralKind::Int {
                        val: "5".to_string(),
                    },
                }),
                Token::new(TokenKind::Whitespace),
                Token::new(TokenKind::Plus),
                Token::new(TokenKind::Whitespace),
                // `(6789.12345 - 600)`
                Token::new(TokenKind::OpenParen),
                Token::new(TokenKind::Lit {
                    kind: LiteralKind::Float {
                        val: "6789.12345".to_string(),
                    },
                }),
                Token::new(TokenKind::Whitespace),
                Token::new(TokenKind::Minus),
                Token::new(TokenKind::Whitespace),
                Token::new(TokenKind::Lit {
                    kind: LiteralKind::Int {
                        val: "600".to_string(),
                    },
                }),
                Token::new(TokenKind::CloseParen),
            ]
            .as_ref()
        )
    )
}
