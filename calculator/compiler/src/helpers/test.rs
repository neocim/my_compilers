use std::fmt::{Debug, Formatter, Result};

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
#[derive(PartialEq, PartialOrd)]
pub struct DebugHelper<T> {
    helper: T,
}

#[derive(PartialEq, PartialOrd)]
pub struct IteratorDebug<T: IntoIterator + Debug>(T);
#[derive(PartialEq, PartialOrd)]
pub struct EnumDebug<T: Debug>(T);

impl<'a, T> DebugHelper<T> {
    pub const fn new_not_iterable(e: T) -> DebugHelper<EnumDebug<T>>
    where
        T: Debug,
    {
        DebugHelper {
            helper: EnumDebug(e),
        }
    }

    pub const fn new_iterable(e: T) -> DebugHelper<IteratorDebug<T>>
    where
        T: IntoIterator + Debug,
    {
        DebugHelper {
            helper: IteratorDebug(e),
        }
    }
}

impl<'a, T> Debug for DebugHelper<IteratorDebug<T>>
where
    T: Debug + IntoIterator<Item: Debug> + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for (i, item) in self.helper.0.clone().into_iter().enumerate() {
            let item = format!("{:?}", item);
            write!(f, "\n{} line (len {}): `{item}`", i + 1, item.len())?;
        }

        Ok(())
    }
}

impl<'a, T> Debug for DebugHelper<EnumDebug<T>>
where
    T: Debug + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let content = format!("{:#?}", self.helper.0);
        for (i, line) in content.lines().enumerate() {
            write!(f, "\n{} line (len {}): `{line}`", i + 1, line.len())?;
        }

        Ok(())
    }
}
