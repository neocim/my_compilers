use crate::{
    lexer::token::LiteralKind,
    span::{Pos, Span},
};

use super::{
    cursor::Cursor,
    token::{Token, TokenKind},
};

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
            // -- `// This function displays 'Hello, world!' in the terminal` --
            Token::new(
                TokenKind::Comment,
                Span::new(Pos::new(1, 1), Pos::new(1, 58))
            ),
            // New line between the comment and the function
            Token::new(
                TokenKind::Whitespace,
                Span::new(Pos::new(1, 58), Pos::new(2, 1))
            ),
            // -- `fn hello_world()` --
            Token::new(TokenKind::Ident, Span::new(Pos::new(2, 1), Pos::new(2, 3))),
            Token::new(
                TokenKind::Whitespace,
                Span::new(Pos::new(2, 3), Pos::new(2, 4))
            ),
            Token::new(TokenKind::Ident, Span::new(Pos::new(2, 4), Pos::new(2, 15))),
            Token::new(
                TokenKind::OpenParen,
                Span::new(Pos::new(2, 15), Pos::new(2, 16))
            ),
            Token::new(
                TokenKind::CloseParen,
                Span::new(Pos::new(2, 16), Pos::new(2, 17))
            ),
            Token::new(
                TokenKind::Whitespace,
                Span::new(Pos::new(2, 17), Pos::new(2, 18))
            ),
            Token::new(
                TokenKind::OpenBrace,
                Span::new(Pos::new(2, 18), Pos::new(2, 19))
            ),
            // New line between the open brace and `print`
            Token::new(
                TokenKind::Whitespace,
                Span::new(Pos::new(2, 19), Pos::new(3, 5))
            ),
            // -- `print("Hello, world!");` --
            Token::new(TokenKind::Ident, Span::new(Pos::new(3, 5), Pos::new(3, 10))),
            Token::new(
                TokenKind::OpenParen,
                Span::new(Pos::new(3, 10), Pos::new(3, 11))
            ),
            Token::new(
                TokenKind::Lit {
                    kind: LiteralKind::Str { terminated: true },
                },
                Span::new(Pos::new(3, 11), Pos::new(3, 26))
            ),
            Token::new(
                TokenKind::CloseParen,
                Span::new(Pos::new(3, 26), Pos::new(3, 27))
            ),
            Token::new(
                TokenKind::SemiColon,
                Span::new(Pos::new(3, 27), Pos::new(3, 28))
            ),
            // New line between semicolon and close brace
            Token::new(
                TokenKind::Whitespace,
                Span::new(Pos::new(3, 28), Pos::new(4, 1))
            ),
            // -- `}` --
            Token::new(
                TokenKind::CloseBrace,
                Span::new(Pos::new(4, 1), Pos::new(4, 2))
            )
        ]
    );
}
