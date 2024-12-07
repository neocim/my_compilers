use crate::{cursor::Cursor, tokenize, LiteralKind, Token, TokenKind};

#[test]
fn check_tokens() {
    let result: Box<[_]> = tokenize("// hello\n101-100 4/2 5%2 2.1+2.2 (2!=3) {10+(5*2)};")
        .map(|token| token)
        .collect();

    assert_eq!(
        result.as_ref(),
        [
            // --- `101-100 ` ---
            Token::new(TokenKind::LineComment),
            Token::new(TokenKind::Whitespace),
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Integer { val: 101 }
            }),
            Token::new(TokenKind::Minus),
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Integer { val: 100 }
            }),
            Token::new(TokenKind::Whitespace),
            // --- `4/2 ` ---
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Integer { val: 4 }
            }),
            Token::new(TokenKind::Slash),
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Integer { val: 2 }
            }),
            Token::new(TokenKind::Whitespace),
            // --- `5%2 ` ---
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Integer { val: 5 }
            }),
            Token::new(TokenKind::Percent),
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Integer { val: 2 }
            }),
            Token::new(TokenKind::Whitespace),
            // --- `2.1+2.2 ` ---
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Float { val: 2.1 }
            }),
            Token::new(TokenKind::Plus),
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Float { val: 2.2 }
            }),
            Token::new(TokenKind::Whitespace),
            // --- `(2!=3) ` ---
            Token::new(TokenKind::OpenParen),
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Integer { val: 2 }
            }),
            Token::new(TokenKind::Bang),
            Token::new(TokenKind::Eq),
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Integer { val: 3 }
            }),
            Token::new(TokenKind::CloseParen),
            Token::new(TokenKind::Whitespace),
            // --- `{10+(5*2)};` ---
            Token::new(TokenKind::OpenBrace),
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Integer { val: 10 }
            }),
            Token::new(TokenKind::Plus),
            Token::new(TokenKind::OpenParen),
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Integer { val: 5 }
            }),
            Token::new(TokenKind::Star),
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Integer { val: 2 }
            }),
            Token::new(TokenKind::CloseParen),
            Token::new(TokenKind::CloseBrace),
            Token::new(TokenKind::Semicolon)
        ]
    );
}

#[test]
fn cursor_test() {
    let mut cursor = Cursor::new("abc");
    assert_eq!(Some('a'), cursor.bump());
    assert_eq!('b', cursor.first());
    assert_eq!('c', cursor.second());
    assert_eq!('\0', cursor.third());
}
