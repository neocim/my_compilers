use std::fmt::Debug;

/// This is a struct for simplifying the debugging of a large number of tokens/something else.
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
pub struct DebugHelper<'a, T>(&'a T)
where
    T: Debug;

impl<'a, T> DebugHelper<'a, T>
where
    T: Debug,
{
    pub fn new(t: &'a T) -> Self {
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
