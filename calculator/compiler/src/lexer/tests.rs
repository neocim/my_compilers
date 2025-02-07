#![cfg(test)]

use crate::{
    helpers::test::DebugHelper,
    lexer::{
        token::{LiteralKind, Token},
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
