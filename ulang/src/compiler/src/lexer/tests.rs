use super::cursor::Cursor;
use crate::lexer::{token::Token, LiteralKind, TokenKind};

fn token_stream(mut cursor: Cursor) -> impl Iterator<Item = Token> + use<'_> {
    std::iter::from_fn(move || {
        let token = cursor.next_token();
        if token.kind != TokenKind::Eof {
            Some(token)
        } else {
            None
        }
    })
}

#[test]
fn cursor_test() {
    let src = "\
// This function displays `Hello, world!` in the terminal
fn hello_world() {
    print(\"Hello, world!\");
}";
    let cursor = Cursor::new(src);
    let expected: Vec<Token> = token_stream(cursor).collect();

    assert_eq!(
        expected,
        vec![
            // -- `// This function displays `Hello, world!` in the terminal` --
            Token::new(TokenKind::Comment, 57),
            // -- `fn hello_world() {` --
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::Ident, 2),
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::Ident, 11),
            Token::new(TokenKind::OpenParen, 1),
            Token::new(TokenKind::CloseParen, 1),
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::OpenBrace, 1),
            Token::new(TokenKind::Whitespace, 5),
            // -- `print(\"Hello, world!\");` --
            Token::new(TokenKind::Ident, 5),
            Token::new(TokenKind::OpenParen, 1),
            Token::new(
                TokenKind::Lit {
                    kind: LiteralKind::Str { terminated: true },
                },
                15
            ),
            Token::new(TokenKind::CloseParen, 1),
            Token::new(TokenKind::SemiColon, 1),
            Token::new(TokenKind::Whitespace, 1),
            // -- `}` --
            Token::new(TokenKind::CloseBrace, 1),
        ]
    );
}
