#![cfg(test)]

use std::fmt::Debug;

use crate::{
    token::{LiteralKind, Token, TokenKind},
    Lexer,
};

#[test]
fn lexer_tests() {
    let input = "123.4 * 5 + (6789.12345 - 600)";
    let mut lexer = Lexer::new(input);
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

/// This is a struct for simplifying the debugging of a large number of tokens.
/// For example, in case of an test error, we will see not this:
/// ```
/// assertion `left == right` failed
///   left: [Token { kind: Lit { kind: Int { val: "2" } } }, Token { kind: Plus }, Token { kind: Whitespace }, Token { kind: Lit { kind: Int { val: "2" } } }]
///   right: [Token { kind: Lit { kind: Int { val: "2" } } }, Token { kind: Whitespace }, Token { kind: Plus }, Token { kind: Whitespace }, Token { kind: Lit { kind: Int { val: "2" } } }]
/// ```
/// but this:
/// ```
/// assertion `left == right` failed
///   left: 1 line: 'Token { kind: Lit { kind: Int { val: "2" } } }'
/// 2 line: 'Token { kind: Whitespace }'
/// 3 line: 'Token { kind: Plus }'
/// 4 line: 'Token { kind: Whitespace }'
/// 5 line: 'Token { kind: Lit { kind: Int { val: "2" } } }'
///
///   right: 1 line: 'Token { kind: Lit { kind: Int { val: "2" } } }'
/// 2 line: 'Token { kind: Plus }'
/// 3 line: 'Token { kind: Whitespace }'
/// 4 line: 'Token { kind: Lit { kind: Int { val: "2" } } }'
/// ```
#[derive(PartialEq, PartialOrd, Clone)]
struct DebugHelper<'a, T>(&'a T)
where
    T: Debug;

impl<'a, T> DebugHelper<'a, T>
where
    T: Debug,
{
    fn new(t: &'a T) -> Self {
        Self(t)
    }
}

impl<'a, T> Debug for DebugHelper<'a, T>
where
    T: Debug + IntoIterator + Clone,
    <T as IntoIterator>::Item: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, item) in self.0.clone().into_iter().enumerate() {
            write!(f, "{} line: '{item:?}'\n", i + 1)?;
        }

        Ok(())
    }
}
