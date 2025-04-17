mod fxset;
#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::fmt::Write as _;

use crate::span::Span;
use fxset::FxIndexSet;

thread_local! {
    static SYMBOL_REGISTRY: RefCell<SymbolRegistryInterner> = RefCell::new(SymbolRegistryInterner::new())
}

impl Symbol {
    pub fn new(idx: u16) -> Self {
        Self(idx)
    }

    pub fn intern(to_intern: String) -> Symbol {
        with_mut_symbol_registry(|symreg| symreg.intern(to_intern))
    }

    pub fn get<'a>(idx: u16) -> Option<String> {
        with_symbol_registry(|symreg| {
            let s = symreg.get(idx);
            s.map(|s| s.to_string())
        })
    }
}

/// Interner for all strings that we are meet during compilation. Here i use `String` instead of
/// `&'static str` because I'm too lazy to write some kind of arena where these variables
/// will live the entire program or look for some other way to make these strings with
/// `'static` lifetime.
struct SymbolRegistryInterner {
    symbols: FxIndexSet<String>,
}

impl SymbolRegistryInterner {
    fn new() -> Self {
        Self {
            symbols: FxIndexSet::default(),
        }
    }

    fn intern(&mut self, to_intern: String) -> Symbol {
        if let Some(idx) = self.symbols.get_index_of(&to_intern) {
            return Symbol::new(idx as u16);
        }
        let (idx, _) = self.symbols.insert_full(to_intern);

        Symbol::new(idx as u16)
    }

    fn get(&self, idx: u16) -> Option<&String> {
        self.symbols.get_index(idx as usize)
    }
}

fn with_symbol_registry<F, R>(func: F) -> R
where
    F: FnOnce(&SymbolRegistryInterner) -> R,
{
    SYMBOL_REGISTRY.with_borrow(func)
}

fn with_mut_symbol_registry<F, R>(func: F) -> R
where
    F: FnOnce(&mut SymbolRegistryInterner) -> R,
{
    SYMBOL_REGISTRY.with_borrow_mut(func)
}

/// Gets string from source text by its span.
/// ### PANIC
/// - ONLY if we passed the wrong span, but our `Cursor` ensures that it will be correct.
/// - Also it can panic if I made a mistakes in the code.
pub fn get_from_src(src: &str, span: Span) -> String {
    let mut result = String::new();
    // Columns
    let l_col = span.lo.col as usize;
    let r_col = span.hi.col as usize;
    // Lines
    let start_l = span.lo.ln as usize;
    let end_l = span.hi.ln as usize;

    // If we need to take several lines, then we will iterate over them.
    if start_l != end_l {
        for (i, line) in src.lines().enumerate().take(end_l.into()).skip(start_l - 1) {
            if i == start_l - 1 {
                let start_byte = line
                    .char_indices()
                    .nth(l_col - 1)
                    .expect("Failed to get the start byte of the string")
                    .0;
                writeln!(result, "{}", &line[start_byte..])
                    .expect("Failed to write line into result string");
            } else if i == end_l - 1 {
                let end_byte = line
                    .char_indices()
                    .nth(r_col - 1)
                    .map(|(end, _)| end)
                    .unwrap_or(line.len());
                write!(result, "{}", &line[..end_byte])
                    .expect("Failed to write line into result string");
            } else {
                writeln!(result, "{}", line).expect("Failed to write line into result string");
            }
        }

        return result;
    }
    // If we are here, then we only need to take one line, so we take it in such a simple way.
    let line = src
        .lines()
        .nth(start_l - 1)
        .expect("Failed to get line by start line");
    let start_byte = src
        .char_indices()
        .nth(l_col - 1)
        .expect("Failed to get start byte of the string")
        .0;
    let end_byte = src
        .char_indices()
        .nth(r_col - 1)
        .map(|(end, _)| end)
        .unwrap_or(line.len());
    write!(result, "{}", &line[start_byte..end_byte])
        .expect("Failed to write line into result string");

    result
}

/// I use `u16` instead of `u32` because I have a small compiler and I
/// doubt that this table will ever be able to fill up with 65535 (max u16 size) symbols.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(u16);
